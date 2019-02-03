use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for '{}' in file: {}...\n", query, filename);

    let contents = fs::read_to_string(filename).expect("read file error");
    println!("\n============================");
    println!("{}", contents);
    println!("============================\n");
}
