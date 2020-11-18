use logos::Logos;
use super::tokens::{ LexToken, TokenKind, Token, Literal };

pub fn get_token_stream(raw_code: &String) -> Vec<Token> {
    let mut token_stream: Vec<Token> = Vec::new();
    let mut lex = LexToken::lexer(raw_code);

    loop {
        let lex_token = lex.next();
        let (kind, value) = match lex_token {
            // Some(LexToken::Error) => panic!("There was an error reading the code"),
            Some(LexToken::Number(val)) => (TokenKind::Number, Some(Literal::Number(val))),
            Some(LexToken::Boolean(val)) => (TokenKind::Boolean, Some(Literal::Boolean(val))),
            Some(LexToken::Text(val)) => (TokenKind::Text, Some(Literal::String(val))),
            Some(LexToken::OpenParen) => (TokenKind::OpenParen, None),
            Some(LexToken::CloseParen) => (TokenKind::CloseParen, None),
            Some(LexToken::Minus) => (TokenKind::Minus, None),
            Some(LexToken::Plus) => (TokenKind::Plus, None),
            Some(LexToken::Divide) => (TokenKind::Divide, None),
            Some(LexToken::Mult) => (TokenKind::Mult, None),
            
            Some(LexToken::Whitespace) => continue,
            Some(LexToken::Error) => continue,
            None => break,
        };

        token_stream.push(Token { kind, value });
    }
    token_stream
}