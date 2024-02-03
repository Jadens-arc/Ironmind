mod tests;
mod turing_machine;
mod parser;
mod visualize;

use turing_machine::Machine;
use parser::Parser;
use clap::Parser as ArgumentParser;

#[derive(ArgumentParser, Debug)]
#[command(author, version)]
#[command(about = "A BrainF*ck Interpreter", long_about = None)]
struct Args {
    filepath: String,
    #[arg(short, long)]
    visual_mode: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let mut p: Parser = Parser::new();
    p.load_file(args.filepath)?;

    if !args.visual_mode {
        // just parse code and display output
        p.parse()?;
        return Ok(());
    } else {
        visualize::Visualizer::visualize(p);
    }
    Ok(())
}
