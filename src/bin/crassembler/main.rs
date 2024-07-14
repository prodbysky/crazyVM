use clap::Parser;
use std::error::Error;

// Casm assembler for the crazyVM VM
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Casm file name to assemble
    #[arg(short, long = "input")]
    input_file: String,

    /// Output filename
    #[arg(short, long = "output")]
    output_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let source = std::fs::read_to_string(args.input_file)?;
    let source = source
        .lines()
        .filter(|line| !line.starts_with(';'))
        .collect::<Vec<_>>();

    Ok(())
}
