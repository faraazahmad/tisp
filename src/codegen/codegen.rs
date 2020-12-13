use std::collections::HashMap;

use crate::tispc_lexer::{Ident, IdentKind, Value};
use crate::tispc_parser::Expr;
use inkwell::context::Context;
use inkwell::{builder::Builder, values::BasicValueEnum};
use inkwell::{module::Module, values::FunctionValue, values::PointerValue};

pub struct Codegen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub builder: &'a Builder<'ctx>,
    pub builtins: &'a mut HashMap<&'a str, FunctionValue<'ctx>>,
    pub variables: &'a mut HashMap<&'a str, PointerValue<'ctx>>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub fn generate_llvm_ir(&mut self, expression_tree: Vec<Expr<'a>>) {
        for expression in expression_tree {
            match expression {
                Expr::Call(func_name_box, args) => match *func_name_box {
                    Expr::Builtin(Ident {
                        kind: IdentKind::FuncName,
                        value: Some(Value::String(func_name)),
                    }) => self.generate_call(func_name, args),
                    _ => (),
                },
                Expr::While { condition, body } => {
                    // TODO: add basic block for loop
                    // TODO: convert condition to llvm format
                    // TODO: add body statements
                    // TODO: add branch condition
                    // TODO: add basic block for code after loop
                }
                _ => (),
            }
        }

        // add return 0 at the end
        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));
    }

    fn generate_args(&self, func_name: &str, args: Vec<Expr>) -> Vec<BasicValueEnum<'ctx>> {
        let mut compiled_args: Vec<BasicValueEnum> = Vec::new();

        for (index, arg) in args.iter().enumerate() {
            let compiled_arg = match arg {
                Expr::Constant(Value::Number(val)) => {
                    BasicValueEnum::FloatValue(self.context.f64_type().const_float(*val))
                }
                Expr::Constant(Value::String(val)) => {
                    let str_name = format!("{}_arg_{}", func_name, index);
                    BasicValueEnum::PointerValue(
                        self.builder
                            .build_global_string_ptr(val, str_name.as_str())
                            .as_pointer_value(),
                    )
                }
                Expr::Builtin(Ident {
                    kind: IdentKind::Variable,
                    value: Some(Value::String(var_name)),
                }) => {
                    let var_ptr = self.variables.get(var_name).unwrap();
                    BasicValueEnum::FloatValue(
                        self.builder
                            .build_load(*var_ptr, var_name)
                            .into_float_value(),
                    )
                }
                _ => panic!("Invalid arg type for function"),
            };

            compiled_args.push(compiled_arg);
        }

        compiled_args
    }

    pub fn generate_call(&mut self, func_name: &str, args: Vec<Expr<'a>>) {
        match func_name {
            "print" => {
                let printf = self.builtins.get("printf").unwrap().clone();
                let mut compiled_args = self.generate_args(func_name, args.clone());
                let format_string = self.generate_printf_format_string(compiled_args.clone());
                compiled_args.insert(0, format_string);
                self.builder
                    .build_call(printf, compiled_args.as_slice(), "printf");
            }
            "let" => {
                // panic if let doesn't have exactly 2 params (name and value)
                if args.len() != 2 {
                    panic!("Invalid syntax for let")
                }

                // get name and value of variable
                let name = match args[0].clone() {
                    Expr::Builtin(Ident {
                        kind: IdentKind::Variable,
                        value: Some(Value::String(val)),
                    }) => val,
                    _ => panic!("Invalid variable name"),
                };
                let value = match args[1].clone() {
                    Expr::Constant(Value::Number(val)) => self.context.f64_type().const_float(val),
                    _ => panic!("Invalid type"),
                };

                let value_ptr = self.builder.build_alloca(self.context.f64_type(), name);
                self.variables.insert(name, value_ptr);
                self.builder.build_store(value_ptr, value);
            }
            _ => panic!("Invalid function: {}", func_name),
        }
    }

    pub fn init(&mut self, source_filename: &str) {
        self.module.set_source_file_name(source_filename);
        self.generate_main_fn();
        self.add_printf();
    }
}
