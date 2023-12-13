use serde::Serialize;

/// Reads dataset from csv and returns vector of the response fields
pub fn read_dataset() -> Vec<String> {
    let mut data = vec![];
    let mut reader = csv::Reader::from_path("dataset_gpt4.csv").unwrap();
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
pub struct FullDistanceComparison {
    pub i: usize,
    pub j: usize,
    pub lev: usize,
    pub jaro: f64,
    pub jac: f64,
    pub dice: f64,
}

#[derive(Serialize)]
pub struct LevComparison {
    pub i: usize,
    pub j: usize,
    pub norm_lev: f64,
}
