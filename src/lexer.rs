use std::{error::Error, fs::File, io::Read};

use crate::token::Token;

/// Take in an input file and produce a list of tokens contained in the file
fn lex_file(filename: String) -> Result<Vec<Token>, Box<dyn Error>> {
    // This vector will hold the individual tokens that are lexed
    let tokens = vec![];

    // Perform neccesary IO to get file contents into a string to be lexed
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{contents}");

    Ok(tokens)
}
