use std::{env, process};

use minigrep::Config;

// Note on println!() vs eprintln!():
// println outputs to stdout or "standard output"
// eprintln outputs to stderr or "standard error"

// Exiting with exit code 1 is customary for exiting with an error

fn main() {
    let args: Vec<String> = env::args().collect();

    // Assigns the output or errors out with customized error
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });
    
    // If the output is an error then errors out with custom error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}