use std::env;
use std::fs;

use inkwell::context::Context;

mod codegen;
use codegen::Codegen;

mod tispc_lexer;
use tispc_lexer::{get_token_stream, Ident};

mod tispc_parser;
use tispc_parser::{generate_expression_tree, Expr};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No file provided");
    }
    let filename = &args[1];
    let raw_code = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let token_stream = get_token_stream(&raw_code);

    // println!("Raw code:\n {}", raw_code);
    // println!("Token stream: \n{:?}", token_stream);
    let expression_tree = generate_expression_tree(token_stream);
    // println!("\n{:?}", expression_tree);

    let context = Context::create();
    let module = context.create_module("example");
    let builder = context.create_builder();
    let codegen = Codegen {
        context: &context,
        module: &module,
        builder: &builder,
    };

    codegen.module.set_source_file_name(filename);
    codegen.generate_main_fn();
    for expression in expression_tree {
        match expression {
            Expr::Call(func_name_box, args) => match *func_name_box {
                Expr::Builtin(Ident::FuncName(func_name)) => codegen.generate_call(func_name, args),
                _ => (),
            },
            _ => (),
        }
    }

    codegen.module.verify().expect("Invalid moduleses");

    println!("{}", codegen.module.print_to_string().to_str().unwrap());
    codegen
        .module
        .print_to_file("/home/faraaz/output.ll")
        .expect("Error printing to file");
}
