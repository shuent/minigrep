use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("err: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("err: {}", e);
        process::exit(1);
    }
}