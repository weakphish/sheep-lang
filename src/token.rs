/// This enumeration describes all possible token types in Rose
pub(crate) enum Token {
    ILLEGAL,       // an illegal token
    EOF,           // end of file
    IDENT(String), // an identifier, like `my_var`
    INT,           // a number, like 1 or 2 or 3
    ASSIGN,        // -
    PLUS,          // +
    MINUS,         // -
    BANG,          // !
    ASTERISK,      // *
    SLASH,         // /
    LT,            // <
    GT,            // >
    COMMA,         // ,
    SEMICOLON,     // ;
    LPAREN,        // (
    RPAREN,        // )
    LBRACE,        // {
    RBRACE,        // }
    FUNCTION,      // fn
    LET,           // let
}

/// Determine if a multi-character keyword is a function or let, and if not, assign it as an
/// identifier.
fn lookup_keyword(ident: &str) -> Token {
    match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        _ => Token::IDENT(ident.to_owned()),
    }
}
