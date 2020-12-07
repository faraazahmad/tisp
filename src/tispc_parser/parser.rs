use crate::tispc_lexer::{Ident, IdentKind, LiteralKind, Token, TokenKind, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Constant(Value<'a>),
    Builtin(Ident<'a>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
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
                _ => panic!("Invalid identifier kind"),
            },
            TokenKind::Literal(LiteralKind::Boolean) => Some(Expr::Constant(token.value.unwrap())),
            TokenKind::Literal(LiteralKind::Number) => Some(Expr::Constant(token.value.unwrap())),
            TokenKind::Literal(LiteralKind::String) => match token.value {
                Some(Value::String(str)) => Some(Expr::Constant(Value::String(str))),
                _ => panic!("Invalid value for string literal"),
            },

            TokenKind::CloseParen => {
                let mut params: Vec<Expr> = Vec::new();
                // pop elements from stack until a FuncName is found
                loop {
                    let expr = stack.pop();
                    params.push(expr.clone().unwrap());
                    match expr {
                        Some(Expr::Builtin(Ident {
                            kind: IdentKind::FuncName,
                            value: _,
                        })) => {
                            break;
                        }
                        _ => continue,
                    }
                }
                let func_name = params.pop().unwrap();

                // reverse params Vec to preserve calling order
                params.reverse();

                // create Expr from params and func name
                Some(Expr::Call(Box::new(func_name), params))
            }

            _ => panic!("Invaid expression"),
        };

        stack.push(expr.unwrap());
    }

    stack
}
