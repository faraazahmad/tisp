use std::collections::HashMap;

use crate::tispc_lexer::{Ident, IdentKind, Value};
use crate::tispc_parser::Expr;
use inkwell::context::Context;
use inkwell::FloatPredicate;
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
                Expr::Call(_, _) => self.generate_call(expression),

                Expr::While { condition, body } => {
                    // TODO: convert condition to llvm format
                    let unboxed_condition = *condition;

                    let (predicate, params) = match unboxed_condition {
                        Expr::Call(boxed_expr, params) => {
                            let expr = *boxed_expr;
                            match expr {
                                Expr::Builtin(Ident {
                                    kind: IdentKind::Greater,
                                    value: _,
                                }) => (FloatPredicate::OGT, params),
                                Expr::Builtin(Ident {
                                    kind: IdentKind::Smaller,
                                    value: _,
                                }) => (FloatPredicate::OLT, params),
                                _ => panic!("Invalid condition expression"),
                            }
                        }
                        _ => panic!("Invalid condition format"),
                    };

                    // NOTE: assume there is only one function (main)
                    let current_fn = self.module.get_function("main");

                    // Compare Basic Block
                    // loads the indexing variable
                    // performs the comparision and jumps to Loop Basic Block
                    // if true, else goes to After Basic Block
                    let comp_bb = self
                        .context
                        .append_basic_block(current_fn.unwrap(), "while_cmp");
                    self.builder.build_unconditional_branch(comp_bb);
                    self.builder.position_at_end(comp_bb);

                    let compiled_cond_params = self.generate_args("while", params);

                    let cond = self.builder.build_float_compare(
                        predicate,
                        compiled_cond_params[0].into_float_value(),
                        compiled_cond_params[1].into_float_value(),
                        "while_cond",
                    );

                    // Loop Basic Block
                    // adds statements to execute in the body
                    // and jumps to Compare basic Block (unconditionally)
                    let loop_bb = self
                        .context
                        .append_basic_block(current_fn.unwrap(), "while");
                    self.builder.position_at_end(loop_bb);

                    // add body statements
                    for expr in body {
                        self.generate_call(expr);
                    }

                    // After Basic Block
                    // basic block for code to run after loop
                    let after_bb = self
                        .context
                        .append_basic_block(current_fn.unwrap(), "after_while");

                    self.builder.build_unconditional_branch(comp_bb);

                    // go to end of Compare Basic Block and add condition
                    self.builder.position_at_end(comp_bb);
                    self.builder
                        .build_conditional_branch(cond, loop_bb, after_bb);

                    // go to end of After Basic Block (end of loop)
                    self.builder.position_at_end(after_bb);
                }
                _ => (),
            }
        }

        // add return 0 at the end
        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));
    }

    fn generate_args(&mut self, func_name: &str, args: Vec<Expr>) -> Vec<BasicValueEnum<'ctx>> {
        let mut compiled_args: Vec<BasicValueEnum> = Vec::new();

        for arg in args {
            let compiled_arg = match arg {
                Expr::Constant(Value::Number(val)) => {
                    BasicValueEnum::FloatValue(self.context.f64_type().const_float(val))
                }
                Expr::Constant(Value::String(val)) => {
                    let str_name = format!("{}_string_arg", func_name);
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
                    let var_ptr_result = self.variables.get(var_name);

                    match var_ptr_result {
                        None => panic!("variable {} not defined", var_name),
                        _ => (),
                    };

                    let var_ptr = var_ptr_result.unwrap();

                    self.builder.build_load(*var_ptr, var_name)
                }
                Expr::Call(func_name_box, params) => {
                    let op_kind = match *func_name_box {
                        Expr::Builtin(Ident {
                            kind: IdentKind::Plus,
                            value: _,
                        }) => IdentKind::Plus,
                        Expr::Builtin(Ident {
                            kind: IdentKind::Minus,
                            value: _,
                        }) => IdentKind::Minus,
                        Expr::Builtin(Ident {
                            kind: IdentKind::Mult,
                            value: _,
                        }) => IdentKind::Mult,
                        Expr::Builtin(Ident {
                            kind: IdentKind::Div,
                            value: _,
                        }) => IdentKind::Div,
                        _ => panic!("Invalid operator type"),
                    };

                    let child_compiled_args = self.generate_args(func_name, params);
                    let (lhs, rhs) = (child_compiled_args[0], child_compiled_args[1]);

                    let result = match op_kind {
                        IdentKind::Plus => self.builder.build_float_add(
                            lhs.into_float_value(),
                            rhs.into_float_value(),
                            "add",
                        ),
                        IdentKind::Minus => self.builder.build_float_sub(
                            lhs.into_float_value(),
                            rhs.into_float_value(),
                            "sub",
                        ),
                        IdentKind::Mult => self.builder.build_float_mul(
                            lhs.into_float_value(),
                            rhs.into_float_value(),
                            "mul",
                        ),
                        IdentKind::Div => self.builder.build_float_div(
                            lhs.into_float_value(),
                            rhs.into_float_value(),
                            "div",
                        ),
                        _ => panic!("Invalid operator type"),
                    };

                    BasicValueEnum::FloatValue(result)
                }
                _ => panic!("Invalid arg type for function {:?}", arg),
            };

            compiled_args.push(compiled_arg);
        }

        compiled_args
    }

    pub fn generate_call(&mut self, expr: Expr<'a>) {
        let (func_name, args) = match expr {
            Expr::Call(func_name_box, params) => match *func_name_box {
                Expr::Builtin(Ident {
                    kind: IdentKind::FuncName,
                    value: Some(Value::String(func_name)),
                }) => (func_name, params),
                _ => panic!("Invalid function call"),
            },
            _ => panic!("Invalid function call"),
        };

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

                let value_vec = vec![args[1].clone()];
                let value = self.generate_args("let", value_vec)[0].into_float_value();

                let val_ptr_result = self.variables.get(name);
                let val_ptr = match val_ptr_result {
                    None => self.builder.build_alloca(self.context.f64_type(), name),
                    _ => *val_ptr_result.unwrap(),
                };
                self.variables.insert(name, val_ptr);
                self.builder.build_store(val_ptr, value);
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
