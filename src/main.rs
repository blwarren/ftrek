mod ftrek;

use crate::ftrek::TrekOptions;
use clap::Parser;

fn main() {
    let options = TrekOptions::parse();

    if let Err(e) = ftrek::run(&options) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
