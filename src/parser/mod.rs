use crate::Machine;
use std::fs::File;
use std::io::{Read, Write};
use std::{io};

// TODO custom error enum

/// Parses brainfuck instructions to manipulate the turing machine
#[derive(Debug, Clone)]
pub struct Parser {
    machine:           Machine,    // The turing machine to operate on
    loops:             Vec<usize>, // Used for handling loops
    instructions:      String,
    instruction_index: usize,      // Current index of input being parsed
    output:            String,     // Collector for bf output
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            machine:            Machine::new(),
            loops:              Vec::new(),
            instructions:       String::new(),
            instruction_index:  0,
            output:             String::new(),
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

    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    pub fn get_instructions(&self) -> String {
        self.instructions.clone()
    }

    pub fn get_instruction(&self, index: usize) -> char {
        self.instructions.chars().nth(index).unwrap()
    }

    pub fn get_current_instruction(&self) -> char {
        self.get_instruction(self.instruction_index.clone())
    }

    pub fn get_instruction_index(&self) -> usize {
        self.instruction_index
    }
    pub fn get_memory(&self) -> Vec<u8> {
        self.machine.get_memory()
    }

    pub fn match_current_instruction(&mut self, silent: bool) -> Result<(), String> {
        self.match_instruction(self.get_current_instruction(), silent)
    }

    pub fn step(&mut self) -> Result<(), String> {
        self.match_current_instruction(false)?;
        self.instruction_index += 1;
        Ok(())
    }

    pub fn match_instruction(&mut self, instruction: char, silent: bool) -> Result<(), String> {
        match instruction {
            '>' => self.machine.move_right(),
            '<' => self.machine.move_left(),
            '+' => self.machine.increment(),
            '-' => self.machine.decrement(),
            '.' => {
                if let Some(char) = self.machine.get_char() {
                    self.output.push(char);
                }
                if !silent {
                    self.machine.output();
                }
            },
            ',' => {
                if let Err(_) = self.get_input() {
                    return Err(String::from("Input could not be parsed"));
                }
            },
            '[' => self.loops.push(self.instruction_index),
            ']' => {
                if self.machine.get() != 0 {
                    self.instruction_index = if let Some(val) = self.loops.last() { val.clone() } else {
                        return Err(String::from("Opening bracket not found"));
                    };
                } else {
                    self.loops.pop();
                };
            },
            _ => (),
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn load(&mut self, input: String) {
        self.instructions = input;
    }

    /// load a BrainFuck file into the instruction set
    pub fn load_file(&mut self, path: String) -> Result<(), String> {
        let mut file: File = if let Ok(file) = File::open(path) { file } else {
            return Err(String::from("Could not open file"))
        };
        if let Err(_) = file.read_to_string(&mut self.instructions) {
            return Err(String::from("Could not read file"))
        }
        Ok(())
    }

    /// Parse a string of brainfuck instructions
    ///
    /// Operates on Turing Machine
    pub fn parse(&mut self) -> Result<String, String> {
        while self.instruction_index < self.instructions.len() {
            self.match_current_instruction(false)?;
            self.instruction_index += 1;
        }
        Ok(self.output.clone())
    }
}

impl Iterator for Parser {
    type Item = Machine;

    fn next(&mut self) -> Option<Self::Item> {
        self.match_current_instruction(true).unwrap();
        self.instruction_index += 1;
        Some(self.machine.clone())
    }
}
