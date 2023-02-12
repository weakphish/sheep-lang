use std::{iter::Peekable, str::CharIndices};

use crate::token::Token;

pub fn lex_input(input: &str) -> Vec<Token> {
    // let chars = input.chars();
    let mut chars = input.char_indices().peekable();
    let mut tokens: Vec<Token> = vec![];

    // start lexing from beginning of chars iterator
    &mut chars.for_each(|c| {
        // check for int
        if c.1.is_ascii_digit() {
            let tok = lex_number(&mut chars);
        }
    });

    tokens
}

/// Find the largest sequence of digits starting at offset and return it as a
/// Token::INT(&str) as well as the amount jumped from the original offset.
#[inline]
fn lex_number<'a>(input: &'a mut Peekable<CharIndices<'a>>) -> Token<'a> {
    /*
    With this, you can have lex_number take an argument of type &mut CharIndices,
    modify it, and just return a Token — and the modifications to the iterator
    in this function will also advance the iterator in lex_input, too.

    There's just one problem: in lex_number, we need to think about the case
    where the next character isn't a number. If it is a number, then we advance
    the iterator, and everything's fine; if it's not, then we return the string
    slice as a token, but we've then used up a character! We need to find some
    way of putting the character "back" in the iterator once we've seen it's not
    a digit. To do this, I would use .peekable() on the iterator, which lets you
    examine the character before opting to advance the iterator if it's the
    character we want.
    */
    let mut number: String = "".into();
    while input.peek().unwrap_or(&(0, '\0')).1.is_alphabetic() {
        // FIXME unwrap_or?
        number.push(input.next().unwrap().1); // FIXME replace unwrap
    }
    Token::Int(&number)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lex_input() {
        let input = "let main() = {puts 12345;}";

        let expected = vec![
            Token::Let,
            Token::Ident("main"),
            Token::LParen,
            Token::RParen,
            Token::Assign,
            Token::LBrace,
            Token::Puts,
            Token::Int("12345"),
            Token::Semicolon,
            Token::RBrace,
        ];

        let result = lex_input(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_lex_number() {
        // TODO
        let input = "12345";

        let result = lex_number(input, 0);

        assert_eq!(result, Token::Int(input));
    }
}
