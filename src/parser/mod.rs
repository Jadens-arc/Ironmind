use crate::Machine;
use std::fs::File;
use std::io::{Read, Write};
use std::{io};
/// Parses brainfuck instructions to manipulate the turing machine
#[derive(Debug, Clone)]
pub struct Parser {
    /// The turing machine to operate on
    machine: Machine,
    /// Used for handling loops
    loops: Vec<usize>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            machine: Machine::new(),
            loops: Vec::new(),
        }
    }

    /// Prompt user to input a char and set the value of the current to the ascii code representation of the char
    pub fn get_input(&mut self) -> Result<(), std::io::Error>{
        print!("> ");
        io::stdout().flush()?;
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;
        if let Some(value) = input.chars().collect::<Vec<char>>().first() {
            self.machine.set(*value as u8);
        }
        Ok(())
    }

    /// Parse a string of brainfuck instructions
    ///
    /// Operates on Turing Machine
    pub fn parse(&mut self, input: String) -> Result<(), String> {
        let letters: Vec<char> = input.chars().collect();
        let mut index: usize = 0;
        while index < letters.len() {
            match letters[index] {
                '>' => self.machine.move_right(),
                '<' => self.machine.move_left(),
                '+' => self.machine.increment(),
                '-' => self.machine.decrement(),
                '.' => self.machine.output(),
                ',' => {
                    if let Err(_) = self.get_input() {
                        return Err(String::from("Input could not be parsed"));
                    }
                },
                '[' => self.loops.push(index),
                ']' => {
                    if self.machine.get() != 0 {
                        index = if let Some(val) = self.loops.last() { *val } else {
                            return Err(String::from("Opening bracket not found"))
                        };
                    } else {
                        self.loops.pop();
                    };
                },
                _ => (),
            }
            index += 1;
        }
        Ok(())
    }

    /// Parse a BrainFuck file and interpret instructions
    ///
    /// Reads file and calls self.parse() to parse its contents
    pub fn parse_file(&mut self, path: String) -> Result<(), String> {
        let mut file: File = if let Ok(file) = File::open(path) { file } else {
            return Err(String::from("Could not open file"))
        };
        let mut contents: String = String::new();
        if let Err(_) = file.read_to_string(&mut contents) {
            return Err(String::from("Could not read file"))
        }
        self.parse(contents)
    }
}

