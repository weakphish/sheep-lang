use crate::token::Token;

/// A lexer contains a vector of characters as well as various pointers to read and peek characters
/// in the input.
pub struct Lexer {
    input: Vec<char>,
    position: usize,      // current reading position
    peek_position: usize, // current peeking position
    ch: char,             // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            peek_position: 1,
            ch: '0',
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let tok = match self.ch {
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::ASTERISK,
            '!' => {
                if self.peek_char()? == '=' {
                    self.read_char(); // advance past '='
                    Token::NEQ
                } else {
                    Token::BANG
                }
            }
            '?' => Token::QUESTION,
            '>' => Token::GT,
            '<' => Token::LT,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '|' => {
                if self.peek_char()? == '>' {
                    self.read_char();
                    Token::PIPE
                } else if self.peek_char()? == '|' {
                    self.read_char();
                    Token::LOR
                } else {
                    // FIXME is there a way around this?
                    Token::ILLEGAL(String::from(self.ch))
                }
            }
            '/' => {
                // TODO add comment support aka skip anything from double // to newline
                if self.peek_char()? == '/' {
                    // need to skip to newline
                    while self.ch != '\n' {
                        self.read_char();
                    }
                    Token::COMMENT // this will be discarded in parsing. maybe keep it for reflection
                } else {
                    Token::SLASH
                }
            }
            '=' => {
                if self.peek_char()? == '=' {
                    self.read_char(); // advance past second '=' sign
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            _ => todo!(),
        };
        self.read_char();
        Some(tok)
    }

    fn read_char(&mut self) {
        self.ch = *self.input.get(self.peek_position).unwrap_or(&'\0'); // FIXME
        self.position = self.peek_position;
    }

    fn peek_char(&self) -> Option<char> {
        // if self.peek_position >= self.input.len() {
        //     '\0'
        // } else {
        //     self.input[l.readPosition]
        // }
        if let Some(ch) = self.input.get(self.peek_position) {
            Some(*ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }
}
