#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token<'a> {
    // Illegal token will store whatever it is that it found
    ILLEGAL(&'a str),
    EOF,
    COMMENT,
    IDENT(&'a str),
    STRING(&'a str),
    INT(&'a str),

    // math
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    BANG,
    QUESTION,
    GT,
    LT,

    // symbols
    COMMA,
    SEMICOLON,
    COLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    EQ,
    NEQ,
    PIPE,
    LET,
    VAR,
    ASSIGN,
    PUTSTR,
    IF,
    ELSE,
    RETURN,

    // logical
    LOR,
    LAND,

    // binary
    BOR,
    BAND,

    // boolean
    TRUE,
    FALSE,
}

/// FIXME optimize me ;)
/// Match a string keyword to a token type. Defaults to IDENT if nothing matches.
pub fn match_keyword(word: &str) -> Token {
    match word {
        "|>" => Token::PIPE,
        "let" => Token::LET,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "putstr" => Token::PUTSTR,
        _ => Token::IDENT(word),
    }
}
