use std::env;
use std::fs;

use inkwell::context::Context;

// mod codegen;
// use codegen::Codegen;

mod tispc_lexer;
use tispc_lexer::get_token_stream;

mod tispc_parser;
use tispc_parser::generate_expression_tree;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No file provided");
    }
    let filename = &args[1];
    let raw_code = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let token_stream = get_token_stream(&raw_code);

    println!("{}", raw_code);
    println!("Token stream: \n{:?}", token_stream);
    let expression_tree = generate_expression_tree(token_stream);
    println!("\n{:?}", expression_tree);

    /* let context = Context::create();
        let module = context.create_module("example");
        let builder = context.create_builder();
        let mut codegen = Codegen {
            context: &context,
            module: &module,
            builder: &builder,
            builtins: &mut Vec::new(),
        };

        codegen.init(filename);
        codegen.generate_llvm_ir(expression_tree);

        // println!("{}", codegen.module.print_to_string().to_str().unwrap());

        codegen.module.verify().expect("Invalid moduleses");

        codegen
            .module
            .print_to_file("/home/faraaz/output.ll")
            .expect("Error printing to file");

    */
}
