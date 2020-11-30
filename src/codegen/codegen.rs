use inkwell::types::BasicTypeEnum;
use inkwell::{builder::Builder, values::BasicValueEnum};
use inkwell::{context::Context, module::Linkage};
use inkwell::{module::Module, values::FunctionValue};

use crate::tispc_lexer::Value;
use crate::tispc_parser::Expr;

pub struct Codegen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub builder: &'a Builder<'ctx>,
    pub builtins: &'a mut Vec<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    // pub fn generate_llvm_ir(&self) {}

    fn generate_args(&self, args: Vec<Expr>) -> Vec<BasicValueEnum<'ctx>> {
        let mut compiled_args: Vec<BasicValueEnum> = Vec::new();

        for arg in args {
            let compiled_arg = match arg {
                Expr::Constant(Value::Number(val)) => {
                    BasicValueEnum::FloatValue(self.context.f64_type().const_float(val))
                }
                Expr::Constant(Value::String(val)) => {
                    BasicValueEnum::PointerValue(
                        self.builder
                            .build_global_string_ptr(val, "nothing")
                            .as_pointer_value(),
                    )
                    // BasicValueEnum::VectorValue(self.context.const_string(val.as_bytes(), true))
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
                let compiled_args = self.generate_args(args);
                self.builder
                    .build_call(printf, compiled_args.as_slice(), "printf");
            }
            _ => panic!("Invalid function: {}", func_name),
        }
    }

    pub fn init(&mut self) {
        self.generate_main_fn();
        self.add_printf();
    }
}
