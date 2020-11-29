use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::{builder::Builder, values::BasicValueEnum};
use inkwell::{context::Context, module::Linkage};

use crate::tispc_lexer::Value;
use crate::tispc_parser::Expr;

pub struct Codegen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub builder: &'a Builder<'ctx>,
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
        let void_type = self.context.void_type();
        let i32_type = self.context.i32_type();
        let str_type = self
            .context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::Generic);

        let mut type_args: Vec<BasicTypeEnum> = Vec::new();
        let compiled_args = self.generate_args(args);
        for arg in compiled_args.clone() {
            type_args.push(arg.get_type());
        }

        let printf_type = i32_type.fn_type(type_args.as_slice(), true);

        // let printf_type = void_type.fn_type(&[BasicTypeEnum::VectorType()], true);

        let printf = self
            .module
            .add_function("printf", printf_type, Some(Linkage::External));

        printf.set_call_conventions(0);

        self.builder
            .build_call(printf, compiled_args.as_slice(), "");
        self.builder
            .build_return(Some(&i32_type.const_int(0, false)));
    }

    pub fn generate_main_fn(&self) {
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[BasicTypeEnum::IntType(i32_type)], false);
        let main = self.module.add_function("main", main_fn_type, None);
        let block = self.context.append_basic_block(main, "entry");
        self.builder.position_at_end(block);
    }
}
