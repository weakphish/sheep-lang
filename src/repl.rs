use std::io;

use crate::lexer::lex_input;

pub fn repl() {
    loop {
        let mut line = String::new();

        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("Could not flush stdout.");
        line.clear(); // First clear the String. Otherwise it will keep adding to it
        io::stdin().read_line(&mut line).unwrap(); // Get the stdin from the user, and put it in read_string

        dbg!(lex_input(&line));
    }
}
