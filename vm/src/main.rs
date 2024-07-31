mod tests;
pub mod utils;

use clap::Parser;
use common::machine::CrazyVM;

use common::machine::RuntimeError;

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
    loop {
        match machine.step() {
            Ok(None) => {}
            Ok(Some(0)) => {
                eprintln!("Program exited succesfully!");
                return;
            }
            Ok(Some(n)) => {
                eprintln!("Program exited abnormally! Exit code: [{}]", n);
                return;
            }
            Err(RuntimeError::NoNextInstruction) => break,
            Err(e) => {
                eprintln!("FATAL ERROR: {}", e);
                break;
            }
        }
    }
    machine.dump_state();
}
