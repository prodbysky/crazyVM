use clap::Parser;
use std::fs::File;
use std::{cmp::Ordering, error::Error, io::Write};

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

    /// Dissasemble the file?
    #[arg(short, long, default_value_t = false)]
    dissasemble: bool,
}

#[derive(Debug)]
enum CompilationError {
    NotEnoughArguments,
    TooManyArguments,
    NonExistantInstruction,
    InvalidRegisterName,
    InvalidNumberLiteral,
}

fn assemble(source: String) -> Result<Vec<u32>, CompilationError> {
    let lines = source
        .lines()
        .filter(|line| !line.starts_with(';'))
        .collect::<Vec<_>>();

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

fn write_binary_to_file(bin: Vec<u32>, file: String) {
    let program: String = bin
        .iter()
        .map(|num| format!("{:08x}", num)) // Format as hex, zero-padded to 8 characters
        .map(|hex_str| hex_str.chars().rev().collect::<String>()) // Reverse the hex string
        .collect::<Vec<String>>()
        .join("\n");

    let mut file = File::create(file).unwrap();
    file.write_all(program.as_bytes()).unwrap();
}

fn dissasemble_to_file(input_file: String, output: String) {
    let program = std::fs::read_to_string(input_file).unwrap();

    let mut instructions: Vec<Opcode> = vec![];

    for line in program.lines() {
        let mut reversed = String::with_capacity(line.len());

        for ch in line.chars().rev() {
            reversed.push(ch);
        }

        instructions.push(Opcode::from(u32::from_str_radix(&reversed, 16).unwrap()));
    }

    let mut output = std::fs::File::create(output).unwrap();

    for ins in instructions {
        output.write_all(format!("{}\n", ins).as_bytes()).unwrap();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let source = std::fs::read_to_string(&args.input_file)?;

    if args.dissasemble {
        dissasemble_to_file(args.input_file, args.output_file);
    } else {
        let program = assemble(source).unwrap();
        write_binary_to_file(program, args.output_file);
    }

    Ok(())
}
