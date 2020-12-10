use std::collections::HashMap;
use std::fs;

use clap::{App, Arg};
use inkwell::context::Context;

mod codegen;
use codegen::Codegen;

mod tispc_lexer;
use tispc_lexer::get_token_stream;

mod tispc_parser;
use tispc_parser::generate_expression_tree;

fn main() {
    let matches = App::new("tispc")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Tisp file to compile"),
        )
        .arg(
            Arg::with_name("emit-llvm")
                .short("e")
                .long("emit-llvm")
                .takes_value(false)
                .help("emits the llvm IR to console"),
        )
        .get_matches();

    let filename = matches
        .value_of("input")
        .expect("Please enter the input file to compile");

    let emit_llvm = matches.is_present("emit-llvm");

    let raw_code = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let token_stream = get_token_stream(&raw_code);

    println!("{}", raw_code);
    println!("Token stream: \n{:?}", token_stream);
    let expression_tree = generate_expression_tree(token_stream);
    println!("\n{:?}", expression_tree);

    let context = Context::create();
    let module = context.create_module("example");
    let builder = context.create_builder();
    let mut codegen = Codegen {
        context: &context,
        module: &module,
        builder: &builder,
        builtins: &mut HashMap::new(),
        variables: &mut HashMap::new(),
    };

    codegen.init(filename);
    codegen.generate_llvm_ir(expression_tree);

    if emit_llvm {
        println!("{}", codegen.module.print_to_string().to_str().unwrap());
    }

    codegen.module.verify().expect("Errors were encountered");

    codegen
        .module
        .print_to_file("/home/faraaz/output.ll")
        .expect("Error printing to file");
}
