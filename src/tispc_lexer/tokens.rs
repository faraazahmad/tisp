use logos::Logos;

// Tokens to be used while running lexer
#[derive(Debug, PartialEq, Logos)]
pub enum LexToken<'a> {
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

    // Match string literals and then strip the " at start and end
    #[regex("\"([^\"\\\\]|\\\\.)*\"")]
    String(&'a str),

    // Non-literal strings and keywords
    #[regex("[a-zA-Z]+")]
    Ident(&'a str),

    #[regex("(-)*([0-9])+", |lex| lex.slice().parse())]
    Number(f64),

    #[regex("(true|false)", |lex| lex.slice().parse())]
    Boolean(bool),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[error]
    Error
}

// Final representation of the kind of token
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    OpenParen,

    CloseParen,

    Comment,
    
    Plus,

    Minus,

    Mult,

    Divide,

    Literal(LiteralKind),

    Ident,

    Whitespace,

    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralKind {
    Number,
    String,
    Boolean
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ident<'a> {
    Variable(&'a str),
    FuncName(&'a str),
    Plus,
    Minus,
    Mult,
    Div
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub value: Option<Value<'a>>
}
