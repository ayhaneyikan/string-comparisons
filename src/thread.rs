use crate::data::read_dataset;
use std::{
    sync::{Arc, Mutex},
    thread,
};

/// Defines which thread payload to use
pub enum Payload {
    /// Performs levenshtein, jaro-winkler, jaccard, and sorensen dice distance computations
    Full,
    /// Performs only normalized levnenshtein similarity computation
    LevOnly,
}

/// Multi-threaded implementation of the string comparisons
pub fn threaded(payload: Payload) {
    let shared_data = Arc::new(read_dataset());
    const STEP_SIZE: usize = 150;
    let done = Arc::new(Mutex::new(0_u32));

    thread::scope(|s| {
        for i in (0..shared_data.len()).step_by(STEP_SIZE) {
            let local_data = shared_data.clone();
            let local_done = done.clone();
            match payload {
                Payload::Full => s.spawn(move || {
                    payloads::process_range_full(
                        i / STEP_SIZE,
                        local_data,
                        i,
                        i + STEP_SIZE,
                        local_done,
                    )
                }),
                Payload::LevOnly => s.spawn(move || {
                    payloads::process_range_lev(
                        i / STEP_SIZE,
                        local_data,
                        i,
                        i + STEP_SIZE,
                        local_done,
                    )
                }),
            };
        }
    });
}

mod payloads {
    use crate::data::{FullDistanceComparison, LevComparison};
    use std::sync::{Arc, Mutex};
    use str_distance::{DistanceMetric, Jaccard, JaroWinkler, Levenshtein, SorensenDice};
    use strsim::normalized_levenshtein;

    /// Thread payload which computes several string distance metrics
    pub fn process_range_full(
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
                let c = FullDistanceComparison {
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
        }
        writer.flush().unwrap();
    }

    /// Thread payload which computes normalized levenshtein similarity only
    pub fn process_range_lev(
        idx: usize,
        data: Arc<Vec<String>>,
        start: usize,
        end: usize,
        done: Arc<Mutex<u32>>,
    ) {
        println!("Thread {} starting...", idx);
        let mut writer = csv::Writer::from_path(format!(
            "gpt4_lev_results/gpt4_norm_lev_threaded_out_{}.csv",
            idx
        ))
        .unwrap();

        for i in start..end {
            // check for early end to data
            if i >= data.len() {
                break;
            }
            for j in (i + 1)..data.len() {
                let c = LevComparison {
                    i,
                    j,
                    norm_lev: normalized_levenshtein(&data[i], &data[j]),
                };
                writer.serialize(c).unwrap();
            }
            let mut counter = done.lock().unwrap();
            *counter += 1;
            println!("Completed {i} | progress: {counter}/{}", data.len());
        }
        writer.flush().unwrap();
    }
}
