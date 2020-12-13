use crate::tispc_lexer::{Ident, IdentKind, LiteralKind, Token, TokenKind, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Constant(Value<'a>),
    Builtin(Ident<'a>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    While {
        condition: Box<Expr<'a>>,
        body: Vec<Expr<'a>>,
    },
}

/// generate_expression_tree
///
/// Takes in a stream of `Token`s and generates an Expression
/// tree of type Vec<Expr>

pub fn generate_expression_tree(token_stream: Vec<Token>) -> Vec<Expr> {
    let mut stack: Vec<Expr> = Vec::new();
    for token in token_stream {
        let expr = match token.kind {
            TokenKind::OpenParen => continue,
            TokenKind::Plus => Some(Expr::Builtin(Ident {
                kind: IdentKind::Plus,
                value: None,
            })),
            TokenKind::Minus => Some(Expr::Builtin(Ident {
                kind: IdentKind::Minus,
                value: None,
            })),
            TokenKind::Mult => Some(Expr::Builtin(Ident {
                kind: IdentKind::Mult,
                value: None,
            })),
            TokenKind::Divide => Some(Expr::Builtin(Ident {
                kind: IdentKind::Div,
                value: None,
            })),
            TokenKind::Ident(ident_kind) => match ident_kind {
                IdentKind::Let => Some(Expr::Builtin(Ident {
                    kind: IdentKind::FuncName,
                    value: Some(Value::String("let")),
                })),
                IdentKind::Print => Some(Expr::Builtin(Ident {
                    kind: IdentKind::FuncName,
                    value: Some(Value::String("print")),
                })),
                IdentKind::Variable => Some(Expr::Builtin(Ident {
                    kind: IdentKind::Variable,
                    value: token.value,
                })),
                IdentKind::While => Some(Expr::Builtin(Ident {
                    kind: IdentKind::While,
                    value: None,
                })),
                _ => panic!("Invalid identifier kind"),
            },
            TokenKind::Literal(LiteralKind::Boolean) => Some(Expr::Constant(token.value.unwrap())),
            TokenKind::Literal(LiteralKind::Number) => Some(Expr::Constant(token.value.unwrap())),
            TokenKind::Literal(LiteralKind::String) => match token.value {
                Some(Value::String(str)) => Some(Expr::Constant(Value::String(str))),
                _ => panic!("Invalid value for string literal"),
            },
            TokenKind::Greater => Some(Expr::Builtin(Ident {
                kind: IdentKind::Greater,
                value: None,
            })),
            TokenKind::Smaller => Some(Expr::Builtin(Ident {
                kind: IdentKind::Smaller,
                value: None,
            })),

            TokenKind::CloseParen => {
                let mut params: Vec<Expr> = Vec::new();
                // pop elements from stack until a Identitfier is found
                loop {
                    let expr = stack.pop();

                    // params.push(expr.clone().unwrap());
                    let expr_clone = expr.clone();
                    match expr {
                        Some(Expr::Builtin(identifier)) => {
                            params.push(expr_clone.unwrap());
                            if identifier.is_builtin() {
                                break;
                            }
                        }
                        None => break,
                        _ => params.push(expr.clone().unwrap()),
                    }
                }

                let lead_ident = params.pop().unwrap();

                let parent_expr = match lead_ident {
                    Expr::Builtin(Ident {
                        kind: IdentKind::While,
                        value: _,
                    }) => {
                        let condition = params.pop().unwrap();
                        // reverse params Vec to preserve expression order
                        params.reverse();
                        Some(Expr::While {
                            condition: Box::new(condition),
                            body: params,
                        })
                    }
                    _ => {
                        // reverse params Vec to preserve expression order
                        params.reverse();
                        Some(Expr::Call(Box::new(lead_ident), params))
                    }
                };

                // create Expr from params and func name
                // Some(Expr::Call(Box::new(func_name), params))
                parent_expr
            } // _ => panic!("Invaid expression"),
        };

        match expr {
            None => (),
            Some(_) => stack.push(expr.unwrap()),
        }
        // stack.push(expr.unwrap());
    }

    stack
}
