use logos::Logos;

#[derive(Debug)]
pub enum Literal<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool)
}

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    value: Option<Literal<'a>>
}

#[derive(Debug, PartialEq, Logos)]
pub enum TokenKind<'a> {
    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Mult,

    #[token("/")]
    Divide,

    #[regex("[a-zA-Z]+")]
    Text(&'a str),

    #[regex("(-)*([0-9])+", |lex| lex.slice().parse())]
    Number(f64),

    #[regex("(true|false)", |lex| lex.slice().parse())]
    Boolean(bool),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[error]
    Error
}

impl TokenKind<'_> {

}

pub fn get_token_stream(raw_code: &String) -> Vec<Token> {
    let mut token_stream: Vec<Token> = Vec::new();
    let mut lex = TokenKind::lexer(raw_code);

    loop {
        let token_kind = lex.next();
        match token_kind {
            // Some(TokenKind::Error) => panic!("There was an error reading the code"),
            None => break,
            Some(TokenKind::Number(val)) => token_stream.push(Token { kind: token_kind.unwrap(), value: Some(Literal::Number(val)) }),
            Some(TokenKind::Boolean(val)) => token_stream.push(Token { kind: token_kind.unwrap(), value: Some(Literal::Boolean(val)) }),
            Some(TokenKind::Text(val)) => token_stream.push(Token { kind: token_kind.unwrap(), value: Some(Literal::String(val)) }),
            _ => token_stream.push(Token { kind: token_kind.unwrap(), value: None }),
        }
    }
    token_stream
}