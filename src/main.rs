// src/main.rs
mod calculator;
mod cli;
mod error;

use cli::Cli;
use clap::Parser;
use std::process;

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = cli.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
