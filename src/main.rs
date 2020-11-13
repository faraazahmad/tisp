use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No file provided");
    }
    let filename = &args[1];
    let raw_code = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("{}", raw_code);
}
