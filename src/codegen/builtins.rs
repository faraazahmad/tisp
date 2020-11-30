use inkwell::module::Linkage;
use inkwell::types::BasicTypeEnum;

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
}
