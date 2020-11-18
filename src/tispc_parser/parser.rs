use crate::tispc_lexer::{ Literal, Token, TokenKind, Ident };

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Constant(Literal<'a>),
    Builtin(Ident<'a>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>)
}

pub fn parse_token_stream(token_stream: Vec<Token>) {
    let mut stack: Vec<Expr> = Vec::new();
    for token in token_stream {
        let expr = match token.kind {
            TokenKind::OpenParen => continue,
            TokenKind::Plus => Some(Expr::Builtin(Ident::Plus)),
            TokenKind::Minus => Some(Expr::Builtin(Ident::Minus)),
            TokenKind::Mult => Some(Expr::Builtin(Ident::Mult)),
            TokenKind::Divide => Some(Expr::Builtin(Ident::Div)),
            TokenKind::Boolean => Some(Expr::Constant(token.value.unwrap())),
            TokenKind::Number => Some(Expr::Constant(token.value.unwrap())),
            TokenKind::Text => {
                match token.value {
                    Some(Literal::String(str)) => Some(Expr::Builtin(Ident::FuncName(str))),
                    _ => panic!("Invalid expression"),
                }
            },

            TokenKind::CloseParen => {
                let mut params: Vec<Expr> = Vec::new();
                loop {
                    let expr = stack.pop();
                    match expr {
                        Some(Expr::Builtin(_)) => {
                            params.push(expr.unwrap());
                            break;
                        },
                        _ => params.push(expr.unwrap()),
                    }
                }
                let func_name = params.pop().unwrap();
                
                // create Expr from params and func name
                Some(Expr::Call(Box::new(func_name), params))
            },

            _ => panic!("Invaid expression")
        };

        stack.push(expr.unwrap());
    }

    println!("\nAST: \n{:?}", stack);
}