use std::error::Error;

use crate::token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lexer {
    input: Vec<char>,     // Source
    position: usize,      // Current position (current char)
    read_position: usize, // current reading position (after current char)
    ch: char,             // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let vector = input.chars().collect();
        Self {
            input: vector,
            position: 0,
            read_position: 0,
            ch: vector.get(0).unwrap().to_owned(),
        }
    }

    /// Take in an input file and produce a list of tokens contained in the file
    pub fn lex_file(&self) -> Result<Vec<Token>, Box<dyn Error>> {
        // This vector will hold the individual tokens that are lexed
        let mut tokens = vec![];

        // TODO Lex the file

        Ok(tokens)
    }

    /// Read a single character
    fn read_char(&mut self) {
        self.ch = match self.input.get(self.read_position) {
            Some(c) => *c,
            None => '\0',
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Skip any whitespace under the 'cursor' of the lexer
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    /// Check if a character is a digit
    fn is_digit(&self, ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    /// Check if a character is a letter
    fn is_letter(&self, ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    /// Grab the next token from the source
    fn next_token(&self) -> Token {
        self.skip_whitespace();
        match self.ch {
            '=' => Token::ASSIGN,
            ';' => Token::SEMICOLON,
            _ => {
                if self.is_letter(self.ch) {
                    // TODO
                } else if self.is_digit(self.ch) {
                    // TODO
                } else {
                    Token::ILLEGAL
                }
            }
        }
    }
}
