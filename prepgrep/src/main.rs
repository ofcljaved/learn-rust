use std::{env, process};
use prepgrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments :{err}");
        process::exit(1);
    });
    if let Err(err) = prepgrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

