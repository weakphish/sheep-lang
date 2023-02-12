#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token<'a> {
    // Illegal token will store whatever it is that it found
    Illegal(&'a str),
    EOF,
    Comment,
    Ident(&'a str),
    String(&'a str),
    Int(&'a str),

    // math
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Question,
    GT,
    LT,

    // symbols
    Comma,
    Semicolon,
    Colon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    EQ,
    NEQ,
    Pipe,
    Let,
    Var,
    Assign,
    Puts,
    If,
    Else,
    Return,

    // logical
    LOr,
    LAnd,

    // binary
    BOr,
    BAnd,

    // boolean
    True,
    False,
}

/// FIXME optimize me ;)
/// Match a string keyword to a token type. Defaults to IDENT if nothing matches.
pub fn match_keyword(word: &str) -> Token {
    match word {
        "|>" => Token::Pipe,
        "let" => Token::Let,
        "if" => Token::If,
        "else" => Token::Else,
        "putstr" => Token::Puts,
        _ => Token::Ident(word),
    }
}
