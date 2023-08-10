use std::process;
use sc::{Config, run};
use clap::Parser;

fn main() {
    // sc/src cargo run -- --word <word>
    // sc/src cargo run -- <file_path>

    let config = Config::parse();

    if let Err(error) = run(&config) {
        eprintln!("An error occurred: {}", error);
        process::exit(1);
    }
}