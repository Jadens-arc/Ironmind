mod tests;
mod turing_machine;
mod parser;
use turing_machine::Machine;
use parser::Parser;
use clap::Parser as ArgumentParser;

#[derive(ArgumentParser, Debug)]
#[command(author, version)]
#[command(about = "A BrainF*ck Interpreter", long_about = None)]
struct Args {
    filepath: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let mut p: Parser = Parser::new();
    p.parse_file(args.filepath)?;
    Ok(())
}
