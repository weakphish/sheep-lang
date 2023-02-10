use crate::{lexer::lex_input, repl::repl};
use std::{env, error::Error, fs};

mod lexer;
mod repl;
pub mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_string = fs::read_to_string(args.get(1).unwrap())?;
        let tokens = lex_input(&file_string);
        dbg!(tokens); // TODO
    } else {
        // REPL Mode
        repl()
    }

    todo!();
}
