use serde::Serialize;
use std::{
    sync::{Arc, Mutex},
    thread::{self},
};
use str_distance::{DistanceMetric, Jaccard, JaroWinkler, Levenshtein, SorensenDice};

/// Reads dataset from csv and returns vector of the response fields
fn read_dataset() -> Vec<String> {
    let mut data = vec![];
    let mut reader = csv::Reader::from_path("dataset.csv").unwrap();
    for r in reader.records() {
        let record = r.unwrap();
        data.push(
            record
                .get(1)
                .unwrap()
                .trim_matches(|c| c == '\'' || c == '\"')
                .to_string(),
        );
    }
    data
}

#[derive(Serialize)]
struct Comparison {
    i: usize,
    j: usize,
    lev: usize,
    jaro: f64,
    jac: f64,
    dice: f64,
}

/// Procedural implementation of the string comparisons
#[allow(dead_code)]
fn procedural() {
    let data = read_dataset();

    let mut writer = csv::Writer::from_path("results/outputs.csv").unwrap();
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            println!("starting {i} {j}");

            let c = Comparison {
                i,
                j,
                lev: *Levenshtein::default().str_distance(&data[i], &data[j]),
                jaro: JaroWinkler::default().str_distance(&data[i], &data[j]),
                jac: Jaccard::new(2).str_distance(&data[i], &data[j]),
                dice: SorensenDice::default().str_distance(&data[i], &data[j]),
            };
            writer.serialize(c).unwrap();
        }
        writer.flush().unwrap();
    }
}

/// Function given to each thread to process a subset of the comparison list
fn process_range(
    idx: usize,
    data: Arc<Vec<String>>,
    start: usize,
    end: usize,
    done: Arc<Mutex<u32>>,
) {
    println!("Thread {} starting...", idx);
    let mut writer =
        csv::Writer::from_path(format!("results/threaded_output_{}.csv", idx)).unwrap();

    for i in start..end {
        // check for early end to data
        if i >= data.len() {
            break;
        }
        for j in (i + 1)..data.len() {
            let c = Comparison {
                i,
                j,
                lev: *Levenshtein::default().str_distance(&data[i], &data[j]),
                jaro: JaroWinkler::default().str_distance(&data[i], &data[j]),
                jac: Jaccard::new(2).str_distance(&data[i], &data[j]),
                dice: SorensenDice::default().str_distance(&data[i], &data[j]),
            };
            writer.serialize(c).unwrap();
        }
        let mut counter = done.lock().unwrap();
        *counter += 1;
        println!("Completed {i} | progress: {counter}/{}", data.len());
        writer.flush().unwrap();
    }
}

/// Multi-threaded implementation of the string comparisons
fn threaded() {
    let shared_data = Arc::new(read_dataset());
    const STEP_SIZE: usize = 150;
    let done = Arc::new(Mutex::new(0_u32));

    thread::scope(|s| {
        for i in (0..shared_data.len()).step_by(STEP_SIZE) {
            let local_data = shared_data.clone();
            let local_done = done.clone();
            s.spawn(move || process_range(i / STEP_SIZE, local_data, i, i + STEP_SIZE, local_done));
        }
    });
}

fn main() {
    // procedural();
    threaded();
}
