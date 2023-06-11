use crate::scanner::Position;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Single character tokens
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    Semicolon,
    Slash,
    Star,
    Question,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,

    //Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Null,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,

    Eof,
}

#[derive(Debug, PartialEq)]
pub struct TokenMetadata {
    pub token: Token,
    pub position: Position,
}
