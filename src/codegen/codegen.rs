use inkwell::context::Context;
use inkwell::{builder::Builder, values::BasicValueEnum};
use inkwell::{module::Module, values::FunctionValue};

use crate::tispc_lexer::{Ident, Value};
use crate::tispc_parser::Expr;

pub struct Codegen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub builder: &'a Builder<'ctx>,
    pub builtins: &'a mut Vec<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub fn generate_llvm_ir(&self, expression_tree: Vec<Expr>) {
        for expression in expression_tree {
            match expression {
                Expr::Call(func_name_box, args) => match *func_name_box {
                    Expr::Builtin(Ident::FuncName(func_name)) => {
                        self.generate_call(func_name, args)
                    }
                    _ => (),
                },
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
                _ => panic!("Invalid arg type for function"),
            };

            compiled_args.push(compiled_arg);
        }

        compiled_args
    }

    pub fn generate_call(&self, func_name: &str, args: Vec<Expr>) {
        match func_name {
            "print" => {
                let printf = self.builtins[0].clone();
                let mut compiled_args = self.generate_args(func_name, args.clone());
                let format_string = self.generate_printf_format_string(compiled_args.clone());
                compiled_args.insert(0, format_string);
                self.builder
                    .build_call(printf, compiled_args.as_slice(), "printf");
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
