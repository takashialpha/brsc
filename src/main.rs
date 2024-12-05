//src/main.rs
mod calculator;
mod cli;
mod error;

// #[cfg(test)]
// mod tests;

/*
Please read TESTING.md for more information about testing
*/

use clap::Parser;
use cli::Cli;
use std::process;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
