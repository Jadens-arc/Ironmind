mod tests;
mod turing_machine;
mod parser;
use turing_machine::Machine;
use parser::Parser;
use std::{env};

fn main() -> Result<(), String> {
    let mut i: Parser = Parser::new();
    i.parse_file(
        match env::args().nth(1) {
            Some(path) => path,
            None => return Err(String::from("Please specify file path")),
        }
    )
}
