mod ftrek;

use crate::ftrek::TrekOptions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let options = TrekOptions::from_args(&args);

    if let Err(e) = ftrek::run(&options) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
