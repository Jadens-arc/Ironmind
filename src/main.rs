mod tests;
mod turing_machine;
mod parser;
mod visualize;

use turing_machine::Machine;
use parser::{Parser, ParserExit};
use clap::Parser as ArgumentParser;
use std::io;
use std::io::Write;

#[derive(ArgumentParser, Debug)]
#[command(author, version)]
#[command(about = "A BrainF*ck Interpreter\nCreated by Jaden Arceneaux\ncontact@jadenarceneaux.com", long_about = None)]
struct Args {
    /// Path to your program
    filepath: String,
    /// Visualize the execution of your program and step through each instruction
    #[arg(short, long)]
    visual_mode: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let mut p: Parser = Parser::new();
    p.load_file(args.filepath)?;

    if args.visual_mode {
        visualize::visualize(p);
        return Ok(());
    } 

    // just parse code and display output
    while p.running() {
        if let Ok(value) = p.match_current_instruction(false) {
            p.increment_instruction_index();
            if value == ParserExit::InputNeeded {
                print!("> ");
                if let Err(_) = io::stdout().flush() {
                    return Err("Could not flush stdout".to_string());
                }
                let mut input: String = String::new();
                if let Err(_) = io::stdin().read_line(&mut input) {
                    return Err("Could not flush read input".to_string());
                }
                if let Some(value) = input.chars().collect::<Vec<char>>().first() {
                    p.set_current_cell(*value as u8)
                }
            }
        }
    }
    Ok(())
}
