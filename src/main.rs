use std::env;
use std::fs;

mod tispc_lexer;
use tispc_lexer::get_token_stream;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No file provided");
    }
    let filename = &args[1];
    let raw_code = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let token_stream = get_token_stream(&raw_code);
    println!("{:?}", token_stream);
}
