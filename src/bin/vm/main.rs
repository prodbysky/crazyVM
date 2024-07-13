pub mod data_structures;
pub mod machine;
mod tests;
pub mod utils;

use clap::Parser;
use machine::CrazyVM;

// crazyVM executable
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// crazyVM bytecode file name to run
    #[arg(short, long = "input")]
    input_file: String,

    /// Memory available to crazyVM
    #[arg(short, long = "mem", default_value_t = 1024 * 1024 * 4)]
    memory_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = match utils::read_binary(&args.input_file) {
        Some(prog) => prog,
        None => {
            eprintln!("Failed to read bytecode file {}", args.input_file);
            return;
        }
    };

    let mut machine = CrazyVM::new(&program, args.memory_size);
    machine.step();
}
