use std::{env, process};
use prepgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments :{err}");
        process::exit(1);
    });
    if let Err(err) = prepgrep::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}

