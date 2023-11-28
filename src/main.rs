mod tests;
mod turing_machine;
mod parser;
use turing_machine::Machine;
use parser::Parser;
use std::{env};

fn main() -> Result<(), String> {
    let mut p: Parser = Parser::new();
    p.parse_file(
        match env::args().nth(1) {
            Some(path) => path,
            None => return Err(String::from("Please specify file path")),
        }
    )
}
