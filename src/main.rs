use std::env;

use procedural::procedural;
use thread::threaded;

mod thread;
mod procedural;
mod data;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args[1].as_str() {
        "-p" => procedural(),
        "-t full" => threaded(thread::Payload::Full),
        "-t lev" => threaded(thread::Payload::LevOnly),
        _ => println!("Usage -p : procedural\n-t full : threaded full comparison\n-t lev : threaded normalized levenshtein comparison only"),
    }
}
