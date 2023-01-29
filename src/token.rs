pub enum Token {
    // Illegal token will store whatever it is that it found
    ILLEGAL(String),
    EOF,
    COMMENT,
    IDENT(String),
    INT(i32),
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    BANG,
    QUESTION,
    GT,
    LT,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    EQ,
    NEQ,
    // Pipe = |>
    PIPE,
    LET,
    VAR,
    ASSIGN,
    IF,
    ELSE,
    RETURN,
    LOR,
    LAND,
    TRUE,
    FALSE,
}

/// Match a string keyword to a token type. Defaults to IDENT if nothing matches.
pub fn match_keyword(word: &str) -> Token {
    match word {
        "|>" => Token::PIPE,
        "let" => Token::LET,
        "if" => Token::IF,
        "else" => Token::ELSE,
        _ => Token::IDENT(word.to_string()),
    }
}
