#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use std::io::Read;
    use crate::Parser;
    #[test]
    pub fn hello_world() -> Result<(), String> {
        let mut p: Parser = Parser::new();
        p.load(String::from("
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
            "));
        while p.running() {
            if let Err(err) = p.match_current_instruction(true) {
                return Err(err);
            }
            p.increment_instruction_index();
        }
        if p.get_output() != "Hello, World!\n".to_string() {
            return Err(format!("Output '{}' invalid", p.get_output()));
        }
        Ok(())
    }

    #[test]
    pub fn bottles_of_beer() -> Result<(), String> {
        let mut p: Parser = Parser::new();
        p.load_file("src/tests/99_bottles_of_beer.bf".to_string())?;
        while p.running() {
            if let Err(err) = p.match_current_instruction(true) {
                return Err(err);
            }
            p.increment_instruction_index();
        }
        let mut file: File = if let Ok(file) = File::open("src/tests/expected_output.txt") { file } else {
            return Err(String::from("Could not open file"));
        };
        let mut contents: String = String::new();
        if let Err(_) = file.read_to_string(&mut contents) {
            return Err(String::from("Could not read file"));
        }
        if p.get_output() != contents {
            return Err(format!("Output '{}' invalid", p.get_output()))
        }
        Ok(())
    }
}
