use std::{env, io};
use std::fs::File;
use std::io::{Read, Write};

/// Defines the tape length for the turing machine
const TAPE_LENGTH: usize = 30000;

/// A turing machine
///
/// Stores data in an array of size TAPE_LENGTH
///
/// Cells in the tap can be modified by a pointer, either by incrementing it or decrementing the cell by 1
/// The pointer can be moved left or right along the tape
#[derive(Debug, Clone)]
struct Machine {
    /// The modifiable array of data
    tape: [u8; TAPE_LENGTH],
    /// the starting location of the pointer
    pointer: usize
}

impl Machine {
    /// Create a new empty Turing Machine
    fn new() -> Machine {
        Machine {
            tape: [0; TAPE_LENGTH],
            pointer: 0,
        }
    }

    /// Move the pointer right
    ///
    /// Only if the pointer is inside the tape
    fn move_right(&mut self) { if self.pointer < TAPE_LENGTH - 1 { self.pointer += 1; } }

    /// Move the pointer to the left
    ///
    /// Only if the pointer is inside the tape
    fn move_left(&mut self) { if self.pointer > usize::MIN { self.pointer -= 1; } }

    /// Increment the current cell (selected by the pointer) by one
    fn increment(&mut self) {
        if self.tape[self.pointer] < u8::MAX {
            self.tape[self.pointer] += 1;
        } else {
            self.tape[self.pointer] = 0;
        }
    }

    /// Decrement the current cell (selected by the pointer) by one
    fn decrement(&mut self) {
        if self.tape[self.pointer] > u8::MIN {
            self.tape[self.pointer] -= 1;
        } else {
            self.tape[self.pointer] = u8::MAX;
        }
    }

    /// Get the unsigned integer value of the current cell
    fn get(&self) -> u8 { self.tape[self.pointer] }

    /// Print the ascii value of the current cell
    fn output(&self) { print!("{}", char::from(self.tape[self.pointer])); }

    /// Set the value of the current cell
    fn set(&mut self, value: u8) {
        self.tape[self.pointer] = value;
    }
}

/// Parses brainfuck instructions to manipulate the turing machine
#[derive(Debug, Clone)]
struct Interpreter {
    /// The turing machine to operate on
    machine: Machine,
    /// Used for handling loops
    loops: Vec<usize>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            machine: Machine::new(),
            loops: Vec::new(),
        }
    }

    /// Prompt user to input a char and set the value of the current to the ascii code representation of the char
    fn get_input(&mut self) -> Result<(), std::io::Error>{
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
    fn parse(&mut self, input: String) -> Result<(), String> {
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
    fn parse_file(&mut self, path: String) -> Result<(), String> {
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

fn main() -> Result<(), String> {
    let mut i: Interpreter = Interpreter::new();
    i.parse_file(
        match env::args().nth(1) {
            Some(path) => path,
            None => return Err(String::from("Please specify file path")),
        }
    )
}

#[test]
fn hello_world() {
    let mut i: Interpreter = Interpreter::new();
    i.parse(String::from("
        ++++++++++++[>++++++<-]>.           H
        >++++++++++[>++++++++++<-]>+.       e
        >+++++++++[>++++++++++++<-]>..      l (printed twice)
        >++++++++++[>+++++++++++<-]>+.      o
        >++++[>+++++++++++<-]>.             (comma)
        >++++[>++++++++<-]>.                (space)
        >++++++++[>+++++++++++<-]>-.        W
        >++++++++++[>+++++++++++<-]>+.      o
        >++++++++++[>+++++++++++<-]>++++.   r
        >+++++++++[>++++++++++++<-]>.       l
        >++++++++++[>++++++++++<-]>.        d
        >++++[>++++++++<-]>+.               (exclamation)
        >+++[>+++<-]>+.                     (new line)
    ")).expect("Hello world test failed");
}