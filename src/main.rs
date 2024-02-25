mod tests;
mod turing_machine;
mod parser;
mod visualize;

use turing_machine::Machine;
use parser::Parser;
use clap::Parser as ArgumentParser;

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

    if !args.visual_mode {
        // just parse code and display output
        while p.running() {
            if let Ok(_) = p.match_current_instruction(false) {
                p.increment_instruction_index();
            }
        }
        return Ok(());
    } 

    visualize::visualize(p);
    Ok(())
}
