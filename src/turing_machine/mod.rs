/// A turing machine
///
/// Stores data in an array of size TAPE_LENGTH
///
/// Cells in the tap can be modified by a pointer, either by incrementing it or decrementing the cell by 1
/// The pointer can be moved left or right along the tape
#[derive(Debug, Clone)]
pub struct Machine {
    /// The modifiable array of data
    tape: Vec<u8>,
    /// the starting location of the pointer
    pointer: usize,
}

impl Machine {
    /// Create a new empty Turing Machine
    pub fn new() -> Machine {
        Machine {
            tape: Vec::from([0]),
            pointer: 0,
        }
    }

    pub fn get_memory(&self) -> Vec<u8> {
        self.tape.clone()
    }

    /// Move the pointer right
    ///
    /// Only if the pointer is inside the tape
    pub fn move_right(&mut self) {
        self.pointer += 1;
        if self.pointer > self.tape.len() - 1 {
            self.tape.push(0);
        }
    }

    /// Move the pointer to the left
    ///
    /// Only if the pointer is inside the tape
    pub fn move_left(&mut self) { if self.pointer > usize::MIN { self.pointer -= 1; } }

    /// Increment the current cell (selected by the pointer) by one
    pub fn increment(&mut self) {
        if let Some(cell) = self.tape.get_mut(self.pointer) {
            *cell = if *cell < u8::MAX { *cell + 1 } else { u8::MIN };
        }
    }

    /// Decrement the current cell (selected by the pointer) by one
    pub fn decrement(&mut self) {
        if let Some(cell) = self.tape.get_mut(self.pointer) {
            *cell = if *cell > u8::MIN { *cell - 1 } else { u8::MAX };
        }
    }

    /// Get the unsigned integer value of the current cell
    pub fn get(&self) -> u8 { self.tape[self.pointer] }

    /// Return the value of the current cell as a char
    pub fn get_char(&self) -> Option<char> {
        if let Some(char) = char::from_u32(self.get() as u32) {
            if char != '\0' {
                return Some(char);
            }
        }
        None
    }

    /// Print the ascii value of the current cell
    pub fn output(&self) {
        if let Some(char) = self.get_char() {
            print!("{}", char);
        }
    }

    /// Set the value of the current cell
    pub fn set(&mut self, value: u8) {
        self.tape[self.pointer] = value;
    }
}
