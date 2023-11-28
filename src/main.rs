mod tests;
mod turing_machine;
mod interpreter;
use turing_machine::Machine;
use interpreter::Interpreter;
use std::{env};

fn main() -> Result<(), String> {
    let mut i: Interpreter = Interpreter::new();
    i.parse_file(
        match env::args().nth(1) {
            Some(path) => path,
            None => return Err(String::from("Please specify file path")),
        }
    )
}
