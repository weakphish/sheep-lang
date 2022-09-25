use std::error::Error;

mod lexer;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename that was sent to the interpeter as a string
    let args: Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap(); // FIXME add better handling with match and print statement

    Ok(())
}
