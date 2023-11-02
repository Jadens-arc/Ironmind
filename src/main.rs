use std::env;
use std::fs::File;
use std::io::Read;

/// Defines the tape length for the turing machine
const TAPE_LENGTH: usize = 30000;

/// A turing machine
///
/// Stores data in an array of size TAPE_LENGTH
///
/// Cells in the tap can be modified by a pointer, either by incrementing it or decrementing the cell by 1
/// The pointer can be moved left or right along the tape
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
    fn move_right(&mut self) { if self.pointer < TAPE_LENGTH { self.pointer += 1; } }

    /// Move the pointer to the left
    ///
    /// Only if the pointer is inside the tape
    fn move_left(&mut self) { self.pointer -= 1; }

    /// Increment the current cell (selected by the pointer) by one
    fn increment(&mut self) { self.tape[self.pointer] += 1; }

    /// Decrement the current cell (selected by the pointer) by one
    fn decrement(&mut self) { self.tape[self.pointer] -= 1; }

    /// Get the unsigned integer value of the current cell
    fn get(&self) -> u8 { self.tape[self.pointer] }

    /// print the ascii value of the current cell
    fn output(&self) { print!("{}", char::from(self.tape[self.pointer] as u8)); }
}

/// Parses brainfuck instructions to manipulate the turing machine
struct Interpreter {
    /// The turing machine to operate on
    machine: Machine,
    /// Used for handling loops
    loop_start: usize,
}

impl Interpreter {
    /// Create a new Interpreter
    fn new() -> Interpreter{
        Interpreter {
            machine: Machine::new(),
            loop_start: 0, // cannot handle nested loops
        }
    }

    /// Parse a string of brainfuck instructions
    ///
    /// Operates on Turing Machine
    fn parse(&mut self, input: String) {
        let mut index: usize = 0;
        while index < input.chars().count() {
            match input.chars().nth(index).expect("out of bounds") {
                '>' => self.machine.move_right(),
                '<' => self.machine.move_left(),
                '+' => self.machine.increment(),
                '-' => self.machine.decrement(),
                '.' => self.machine.output(),
                '[' => self.loop_start = index,
                ']' => index = if self.machine.get() != 0 { self.loop_start } else {index},
                _ => (),
            }
            index += 1;
        }
    }

    /// Parse a BrainFuck file and interpret instructions
    ///
    /// Reads file and calls self.parse() to parse its contents
    fn parse_file(&mut self, path: String) {
        let mut file: File = File::open(path).expect("could not open file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).expect("could not read file");
        self.parse(contents);
    }
}

fn main() {
    let mut i: Interpreter = Interpreter::new();
    i.parse_file(env::args().nth(1).expect("Please specify file to interpret"));
}