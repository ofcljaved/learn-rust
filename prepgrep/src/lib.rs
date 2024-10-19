use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    println!("{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file = args[2].clone();
        Ok(Config { query, file })
    }
}
