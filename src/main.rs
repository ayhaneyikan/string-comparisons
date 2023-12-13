use std::env;

use procedural::procedural;
use thread::threaded;

mod data;
mod procedural;
mod thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "-p" {
        procedural();
    } else if args.len() == 3 {
        if args[2] == "full" {
            threaded(thread::Payload::Full);
            return;
        }
        if args[2] == "lev" {
            threaded(thread::Payload::LevOnly);
            return;
        }
    }

    println!("Usage\t-p\t: procedural\n\t-t full\t: threaded full comparison\n\t-t lev\t: threaded normalized levenshtein comparison only");
}
