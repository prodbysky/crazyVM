use clap::Parser;
use std::{cmp::Ordering, error::Error, fs::File, io::Write};

use common::{
    instructions::{Bit13Literal, Opcode},
    registers::Register,
};

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

#[derive(Debug)]
enum CompilationError {
    NotEnoughArguments,
    TooManyArguments,
    NonExistantInstruction,
    InvalidRegisterName,
    InvalidNumberLiteral,
}

fn assemble(lines: &[&str]) -> Result<Vec<u32>, CompilationError> {
    let mut buffer = vec![];

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "Add" => {
                match parts.len().cmp(&4) {
                    Ordering::Less => return Err(CompilationError::NotEnoughArguments),
                    Ordering::Equal => {}
                    Ordering::Greater => return Err(CompilationError::TooManyArguments),
                }
                let r1 = match Register::try_from(parts[1]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                let r2 = match Register::try_from(parts[2]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                let r3 = match Register::try_from(parts[3]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };

                buffer.push(Opcode::Add(r1, r2, r3).into())
            }
            "Sub" => {
                match parts.len().cmp(&4) {
                    Ordering::Less => return Err(CompilationError::NotEnoughArguments),
                    Ordering::Equal => {}
                    Ordering::Greater => return Err(CompilationError::TooManyArguments),
                }
                let r1 = match Register::try_from(parts[1]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                let r2 = match Register::try_from(parts[2]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                let r3 = match Register::try_from(parts[3]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };

                buffer.push(Opcode::Sub(r1, r2, r3).into())
            }
            "Mul" => {
                match parts.len().cmp(&4) {
                    Ordering::Less => return Err(CompilationError::NotEnoughArguments),
                    Ordering::Equal => {}
                    Ordering::Greater => return Err(CompilationError::TooManyArguments),
                }
                let r1 = match Register::try_from(parts[1]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                let r2 = match Register::try_from(parts[2]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                let r3 = match Register::try_from(parts[3]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };

                buffer.push(Opcode::Mul(r1, r2, r3).into())
            }
            "Div" => {
                match parts.len().cmp(&4) {
                    Ordering::Less => return Err(CompilationError::NotEnoughArguments),
                    Ordering::Equal => {}
                    Ordering::Greater => return Err(CompilationError::TooManyArguments),
                }
                let r1 = match Register::try_from(parts[1]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                let r2 = match Register::try_from(parts[2]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                let r3 = match Register::try_from(parts[3]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };

                buffer.push(Opcode::Div(r1, r2, r3).into())
            }
            "Imm" => {
                match parts.len().cmp(&3) {
                    Ordering::Less => return Err(CompilationError::NotEnoughArguments),
                    Ordering::Equal => {}
                    Ordering::Greater => return Err(CompilationError::TooManyArguments),
                }
                let register = match Register::try_from(parts[1]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };

                let imm_value = match Bit13Literal::try_from(parts[2]) {
                    Ok(v) => v,
                    Err(_) => return Err(CompilationError::InvalidNumberLiteral),
                };

                buffer.push(Opcode::Imm(register, imm_value).into())
            }
            "Push" => {
                match parts.len().cmp(&2) {
                    Ordering::Less => return Err(CompilationError::NotEnoughArguments),
                    Ordering::Equal => {}
                    Ordering::Greater => return Err(CompilationError::TooManyArguments),
                }
                let register = match Register::try_from(parts[1]) {
                    Ok(r) => r,
                    Err(_) => return Err(CompilationError::InvalidRegisterName),
                };
                buffer.push(Opcode::Push(register).into())
            }
            _ => return Err(CompilationError::NonExistantInstruction),
        }
    }

    Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let source = std::fs::read_to_string(args.input_file)?;
    let source = source
        .lines()
        .filter(|line| !line.starts_with(';'))
        .collect::<Vec<_>>();

    let program = assemble(&source).unwrap();
    let program: String = program
        .iter()
        .map(|num| format!("{:08x}", num)) // Format as hex, zero-padded to 8 characters
        .map(|hex_str| hex_str.chars().rev().collect::<String>()) // Reverse the hex string
        .collect::<Vec<String>>()
        .join(" ");

    let mut file = File::create(args.output_file)?;
    file.write_all(program.as_bytes())?;

    Ok(())
}
