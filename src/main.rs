use std::{env, error::Error, fs};
use crate::token::Token;

mod lexer;
pub mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
//    let file_string = fs::read_to_string(args.get(1).unwrap())?;
//    let mut lexer = lexer::Lexer::new(file_string);

    // TODO
    let input: String = "let main() = {\nputs\"hello\" // print is a primitive statement, standard // to denote comment".to_owned();
    let expected: Vec<Token> = vec![
        Token::LET,
        Token::IDENT("main".to_owned()),
        Token::ASSIGN,
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACE,
        Token::PUTSTR,
        Token::STRING(
            "print is a primitive statement, standard // to denote comment".to_owned(),
        ),
    ];

    let mut lexer = lexer::Lexer::new(input);

    let mut got: Vec<Token> = vec![];
    while let Some(t) = lexer.next_token() {
        got.push(t);
    }

    assert!(got == expected);

    Ok(())
}
