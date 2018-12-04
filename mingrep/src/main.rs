extern crate mingrep;

use std::env;
use std::process;

use mingrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("In file {}\nSearching for: {}\n", config.filename, config.query);

    if let Err(e) = mingrep::run(config) {
        eprintln!("mingrep: error: {}", e);
        process::exit(1);
    }
}
