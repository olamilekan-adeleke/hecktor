use std::process;

use hecktor::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("-------- Problem parsing arguments -------- \n{}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application stop due to error: {}", err);
        process::exit(1);
    }
}
