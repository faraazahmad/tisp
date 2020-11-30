use inkwell::module::Linkage;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

use crate::codegen::Codegen;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub fn generate_main_fn(&self) {
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[BasicTypeEnum::IntType(i32_type)], false);
        let main = self.module.add_function("main", main_fn_type, None);
        let block = self.context.append_basic_block(main, "entry");
        self.builder.position_at_end(block);
    }

    // define and add the printf function to the module
    pub fn add_printf(&mut self) {
        let void_type = self.context.void_type(); // return type
        let str_type = self
            .context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::Generic);
        let printf_args_type = vec![BasicTypeEnum::PointerType(str_type)];
        let printf_type = void_type.fn_type(printf_args_type.as_slice(), true);

        let printf_fn = self
            .module
            .add_function("printf", printf_type, Some(Linkage::External));

        self.builtins.push(printf_fn);
    }

    pub fn generate_printf_format_string(
        &self,
        compiled_args: Vec<BasicValueEnum<'ctx>>,
    ) -> BasicValueEnum<'ctx> {
        let mut format_string = String::from("");

        for arg in compiled_args {
            let format_arg = match arg {
                BasicValueEnum::FloatValue(_) => "%f",
                BasicValueEnum::PointerValue(_) => "%s",
                _ => panic!("Invalid arg type for printf"),
            };

            format_string.push_str(format_arg);
        }

        BasicValueEnum::PointerValue(
            self.builder
                .build_global_string_ptr(format_string.as_str(), "format_string")
                .as_pointer_value(),
        )
    }
}
