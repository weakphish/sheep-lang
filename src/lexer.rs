use crate::token::Token;

pub fn lex_input(input: &str) -> Vec<Token> {
    let chars = input.chars();
    let mut tokens: Vec<Token> = vec![];

    dbg!(&input);
    dbg!(&chars);

    // start lexing from beginning of chars iterator
    let mut pos = 0;
    for c in chars {
        // check for int
        if c.is_ascii_digit() {
            dbg!("in digit");
            let (tok, jump) = lex_number(input, pos);

            dbg!(&tok);
            dbg!(&jump);

            pos = pos + jump;
            tokens.push(tok);

            dbg!(pos);
        } else if c.is_alphabetic() {
            dbg!("in alphabetic");
        } else {
            dbg!("BREAKING!");
            break; // TODO
        }
        pos += 1;
    }

    todo!();
}

/// Find the largest sequence of digits starting at offset and return it as a
/// Token::INT(&str) as well as the amount jumped from the original offset.
#[inline]
fn lex_number(input: &str, offset: usize) -> (Token, usize) {
    dbg!("lexing number");

    let mut peek = offset + 1;
    while let Some(i) = input.as_bytes().get(peek) {
        let c = *i as char;
        if (c).is_ascii_digit() {
            peek += 1
        } else {
            break;
        }
    }
    (Token::INT(&input[offset..peek]), (peek - offset))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lex_input() {
        let input = "let main() = {puts 12345;}";

        let expected = vec![
            Token::LET,
            Token::IDENT("main"),
            Token::LPAREN,
            Token::RPAREN,
            Token::ASSIGN,
            Token::LBRACE,
            Token::PUTSTR,
            Token::INT("12345"),
            Token::SEMICOLON,
            Token::RBRACE,
        ];

        let result = lex_input(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_lex_number() {
        // TODO
        let input = "12345";

        let result = lex_number(input, 0);

        assert_eq!(result, Token::INT(input));
    }
}
