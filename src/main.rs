use std::error::Error;

use crate::lexer::lex_file;

mod lexer;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename that was sent to the interpeter as a string
    let args: Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap(); // FIXME add better handling with match and print statement

    // Lex the file into tokens
    let tokens = lex_file(file)?; // FIXME add better error handling here

    // FIXME debug
    dbg!(tokens);

    Ok(())
}
