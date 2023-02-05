use crate::token::Token;
use std::{env, error::Error, fs};

mod lexer;
pub mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_string = fs::read_to_string(args.get(1).unwrap())?;

    todo!();

    Ok(())
}
