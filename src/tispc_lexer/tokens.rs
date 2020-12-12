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

    #[token("let")]
    Let,

    #[token("print")]
    Print,

    // Match string literals and then strip the " at start and end
    #[regex("\"([^\"\\\\]|\\\\.)*\"")]
    String(&'a str),

    // Non-literal strings (currently variable names)
    #[regex("[a-zA-Z]+")]
    Ident(&'a str),

    #[regex("-?([0-9])+", |lex| lex.slice().parse())]
    Number(f64),

    #[regex("(true|false)", |lex| lex.slice().parse())]
    Boolean(bool),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}

// Final representation of the kind of token
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    OpenParen,

    CloseParen,

    Plus,

    Minus,

    Mult,

    Divide,

    Literal(LiteralKind),

    Ident(IdentKind),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralKind {
    Number,
    String,
    Boolean,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentKind {
    Variable,
    Let,
    Print,
    FuncName,
    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident<'a> {
    pub kind: IdentKind,
    pub value: Option<Value<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub value: Option<Value<'a>>,
}
