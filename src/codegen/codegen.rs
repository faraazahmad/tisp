use std::collections::HashMap;

use crate::tispc_lexer::{Ident, IdentKind, Value};
use crate::tispc_parser::Expr;
use inkwell::context::Context;
use inkwell::values::{FloatValue, FunctionValue};
use inkwell::FloatPredicate;
use inkwell::{builder::Builder, values::BasicValueEnum};
use inkwell::{module::Module, values::PointerValue};

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
            match self.compile_expr(expression) {
                Err(x) => panic!("{}", x),
                _ => (),
            }
        }

        // add return 0 at the end
        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));
    }

    fn compile_expr(&mut self, expression: Expr<'a>) -> Result<FloatValue<'ctx>, &'a str> {
        match expression {
            Expr::Call(_, _) => self.compile_call(expression),

            Expr::Constant(Value::Number(val)) => Ok(self.context.f64_type().const_float(val)),

            Expr::Builtin(Ident {
                kind: IdentKind::Variable,
                value: Some(Value::String(val)),
            }) => match self.variables.get(val) {
                Some(var) => Ok(self.builder.build_load(*var, val).into_float_value()),
                None => Err("Could not find variable"),
            },

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

                let lhs = self.compile_expr(params[0].clone())?;
                let rhs = self.compile_expr(params[1].clone())?;

                let cond = self
                    .builder
                    .build_float_compare(predicate, lhs, rhs, "while_cond");

                // Loop Basic Block
                // adds statements to execute in the body
                // and jumps to Compare basic Block (unconditionally)
                let loop_bb = self
                    .context
                    .append_basic_block(current_fn.unwrap(), "while");
                self.builder.position_at_end(loop_bb);

                // add body statements
                for expr in body {
                    match self.compile_expr(expr) {
                        Err(x) => return Err(x),
                        _ => (),
                    }
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

                Ok(self.context.f64_type().const_float(0.0))
            }
            _ => Err("Invalid expression. can\'t compile"),
        }
    }

    fn compile_call(&mut self, expr: Expr<'a>) -> Result<FloatValue<'ctx>, &'a str> {
        let expression = expr.clone();

        match expression {
            Expr::Call(boxed_func_name, _params) => match *boxed_func_name {
                Expr::Builtin(_) => self.compile_builtin(expr.clone()),
                _ => Err("Invalid call expression"),
            },
            _ => Err("Internal error: Invalid use of compile_call function"),
        }
    }

    fn compile_builtin(&mut self, expr: Expr<'a>) -> Result<FloatValue<'ctx>, &'a str> {
        let (func_name_ident, args) = match expr {
            Expr::Call(func_name_box, params) => match *func_name_box {
                Expr::Builtin(func_name_ident) => (func_name_ident, params),
                _ => panic!("Invalid builtin function {:?}", *func_name_box),
            },
            _ => panic!("Invalid function call {:?}", expr),
        };

        match func_name_ident {
            Ident {
                kind: IdentKind::Print,
                value: _,
            } => {
                let printf = self.builtins.get("printf").unwrap().clone();

                // let mut compiled_args = self.generate_args("print", args.clone());
                // let format_string = self.generate_printf_format_string(compiled_args.clone());
                // compiled_args.insert(0, format_string);

                let mut compiled_args: Vec<FloatValue<'ctx>> = Vec::new();
                for arg in args {
                    compiled_args.push(self.compile_expr(arg)?);
                }

                let mut argsv: Vec<BasicValueEnum<'ctx>> = compiled_args
                    .iter()
                    .by_ref()
                    .map(|&val| val.into())
                    .collect();

                let format_string = self.generate_printf_format_string(argsv.clone());
                argsv.insert(0, format_string);

                match self
                    .builder
                    .build_call(printf, argsv.as_slice(), "printf")
                    .try_as_basic_value()
                    .left()
                {
                    Some(value) => Ok(value.into_float_value()),
                    None => Err("Invalid call to print"),
                }
            }
            Ident {
                kind: IdentKind::Let,
                value: _,
            } => {
                // panic if let doesn't have exactly 2 params (name and value)
                if args.len() != 2 {
                    return Err("Invalid syntax for let");
                }

                // get name and value of variable
                let name = match args[0].clone() {
                    Expr::Builtin(Ident {
                        kind: IdentKind::Variable,
                        value: Some(Value::String(val)),
                    }) => val,
                    _ => panic!("Invalid variable name"),
                };

                // let value_vec = vec![args[1].clone()];
                // let value = self.generate_args("let", value_vec)[0].into_float_value();
                let value = self.compile_expr(args[1].clone());

                let val_ptr_result = self.variables.get(name);
                let val_ptr = match val_ptr_result {
                    None => self.builder.build_alloca(self.context.f64_type(), name),
                    _ => *val_ptr_result.unwrap(),
                };
                self.variables.insert(name, val_ptr);
                self.builder.build_store(val_ptr, value.unwrap());

                value
            }
            Ident {
                kind: IdentKind::Plus,
                value: None,
            }
            | Ident {
                kind: IdentKind::Minus,
                value: None,
            }
            | Ident {
                kind: IdentKind::Mult,
                value: None,
            }
            | Ident {
                kind: IdentKind::Div,
                value: None,
            } => {
                let mut argsv = args.clone();

                let mut result = self.compile_expr(argsv.pop().unwrap())?;

                match func_name_ident {
                    Ident {
                        kind: IdentKind::Plus,
                        value: None,
                    } => {
                        for arg in argsv {
                            result = self.builder.build_float_add(
                                result,
                                self.compile_expr(arg)?,
                                "add",
                            );
                        }
                    }
                    Ident {
                        kind: IdentKind::Minus,
                        value: None,
                    } => {
                        for arg in argsv {
                            result = self.builder.build_float_sub(
                                result,
                                self.compile_expr(arg)?,
                                "add",
                            );
                        }
                    }
                    Ident {
                        kind: IdentKind::Mult,
                        value: None,
                    } => {
                        for arg in argsv {
                            result = self.builder.build_float_mul(
                                result,
                                self.compile_expr(arg)?,
                                "add",
                            );
                        }
                    }
                    Ident {
                        kind: IdentKind::Div,
                        value: None,
                    } => {
                        for arg in argsv {
                            result = self.builder.build_float_div(
                                result,
                                self.compile_expr(arg)?,
                                "add",
                            );
                        }
                    }
                    _ => return Err("Invalid operation"),
                }

                Ok(result)
            }
            _ => Err("function not defined."),
        }
    }

    pub fn init(&mut self, source_filename: &str) {
        self.module.set_source_file_name(source_filename);
        self.generate_main_fn();
        self.add_printf();
    }
}
