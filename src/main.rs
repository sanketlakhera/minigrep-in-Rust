use minigrep::Cli;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Cli::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("in file {}", config.file);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
