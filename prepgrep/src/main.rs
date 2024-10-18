use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args.get(1)
        .expect("Pass the word to query");
    let file = &args.get(2)
        .expect("Pass the file name to grep query");
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");
    println!("{}", contents);
}
