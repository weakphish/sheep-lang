use std::str::Chars;

use crate::token::{match_keyword, Token};

pub fn lex_input(input: &str) {
    let mut peek = 0;

    let chars = input.chars();
    let tokens: Vec<Token> = vec![];

    // start lexing from beginning of chars iterator
    for (i, c) in chars.enumerate() {
        match c {
            // is it a number? oooh who knows!!!
            c if c.is_digit(10) => tokens.push(Token::INT(lex_number(input, i).unwrap())), // FIXME better error handling
            _ => todo!(),
        }
    }

    todo!();
}

fn lex_number(input: &str, offset: usize) -> Option<&str> {
    let mut peek = offset;
    while input.get(peek)?.is_digit() {
        peek += 1;
    }

    Some(&input[offset..peek]);

    todo!();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lexer() {
        // TODO
        let input = "let main() = {\nputs\"hello\" // print is a primitive statement, standard // to denote comment";
        let expected: Vec<Token> = vec![
            Token::LET,
            Token::IDENT("main"),
            Token::ASSIGN,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::PUTSTR,
            Token::STRING("print is a primitive statement, standard // to denote comment"),
        ];
    }
}
