use crate::data::{read_dataset, FullDistanceComparison};
use str_distance::{Jaccard, JaroWinkler, Levenshtein, SorensenDice, DistanceMetric};

/// Procedural implementation of the string comparisons
#[allow(dead_code)]
pub fn procedural() {
    let data = read_dataset();

    let mut writer = csv::Writer::from_path("results/outputs.csv").unwrap();
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            println!("starting {i} {j}");

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
        writer.flush().unwrap();
    }
}