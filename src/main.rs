use std::env;

use rust_playground::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args.as_slice()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(s) = rust_playground::run(&config) {
        eprintln!("Application error: {s} \n{config:#?}");
        std::process::exit(1);
    }
}