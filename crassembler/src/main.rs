use clap::Parser;
use core::fmt;
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

struct CompError(Line, u32, &'static str, String);

impl fmt::Display for CompError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, _) = (self.0 .0[self.1 as usize].x, self.0 .0[self.1 as usize].y);
        writeln!(f, "{}:", self.3)?;
        writeln!(f, "{}", self.2)?;
        writeln!(f, "   {}", self.0)?;
        write!(f, "   ")?;
        for _ in 0..x + 1 {
            write!(f, " ")?;
        }
        write!(f, "^")?;

        Ok(())
    }
}

#[derive(Clone)]
struct Token {
    value: String,
    x: u32,
    y: u32,
}

fn token_from_line(line: &str, x: &mut usize, y: u32) -> Token {
    let mut buffer = String::new();
    let begin = *x;

    while let Some(c) = line.chars().nth(*x) {
        if c.is_whitespace() {
            break;
        }

        buffer.push(c);
        *x += 1;
    }

    Token {
        value: buffer,
        x: begin as u32,
        y,
    }
}

#[derive(Clone)]
struct Line(Vec<Token>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .fold(String::new(), |mut prev, curr| {
                    prev.push_str(format!(" {}", curr.value).as_str());
                    prev
                })
                .to_owned()
        )
    }
}

fn tokenize(source: String) -> Vec<Line> {
    let lines: Vec<(usize, &str)> = source
        .lines()
        .filter(|l| !l.starts_with(';'))
        .map(|l| l.trim())
        .enumerate()
        .collect();
    let mut tokens = vec![];

    for line in lines {
        let mut token_line = vec![];

        let mut x = 0;

        while x < line.1.len() {
            if line.1.chars().nth(x).is_some_and(|c| c.is_whitespace()) {
                x += 1;
                continue;
            }
            token_line.push(token_from_line(line.1, &mut x, line.0 as u32));
            x += 1; // Skip whitespace after token
        }
        tokens.push(Line(token_line));
    }

    tokens
}

fn assemble(file_name: String, source: String) -> Result<Vec<u32>, CompError> {
    let tokens = tokenize(source);

    let mut buffer = vec![];

    let err_from_ordering = |ordering: Ordering, line: &Line, file: &str| match ordering {
        Ordering::Less => Err(CompError(
            line.clone(),
            0,
            "Not enough arguments provided",
            file_name.clone(),
        )),
        Ordering::Equal => Ok(()),
        Ordering::Greater => Err(CompError(
            line.clone(),
            0,
            "Too many arguments provided",
            file.to_owned(),
        )),
    };

    let get_reg_or_ret = |idx: usize, line: &Line, file: &str| -> Result<Register, CompError> {
        Register::try_from(line.0[idx].value.as_str()).map_err(|_| {
            CompError(
                line.clone(),
                idx as u32,
                "Invalid register name",
                file.to_string(),
            )
        })
    };

    for line in &tokens {
        let line = line.clone();
        if line.0.is_empty() {
            continue;
        }

        match line.0[0].value.split_whitespace().collect::<Vec<_>>()[0] {
            "Add" => {
                err_from_ordering(line.0.len().cmp(&4), &line, &file_name)?;
                let r1 = get_reg_or_ret(1, &line, &file_name)?;
                let r2 = get_reg_or_ret(2, &line, &file_name)?;
                let r3 = get_reg_or_ret(3, &line, &file_name)?;

                buffer.push(Opcode::Add(r1, r2, r3).into())
            }
            "Sub" => {
                err_from_ordering(line.0.len().cmp(&4), &line, &file_name)?;
                let r1 = get_reg_or_ret(1, &line, &file_name)?;
                let r2 = get_reg_or_ret(2, &line, &file_name)?;
                let r3 = get_reg_or_ret(3, &line, &file_name)?;

                buffer.push(Opcode::Sub(r1, r2, r3).into())
            }
            "Mul" => {
                err_from_ordering(line.0.len().cmp(&4), &line, &file_name)?;
                let r1 = get_reg_or_ret(1, &line, &file_name)?;
                let r2 = get_reg_or_ret(2, &line, &file_name)?;
                let r3 = get_reg_or_ret(3, &line, &file_name)?;

                buffer.push(Opcode::Mul(r1, r2, r3).into())
            }
            "Div" => {
                err_from_ordering(line.0.len().cmp(&4), &line, &file_name)?;
                let r1 = get_reg_or_ret(1, &line, &file_name)?;
                let r2 = get_reg_or_ret(2, &line, &file_name)?;
                let r3 = get_reg_or_ret(3, &line, &file_name)?;

                buffer.push(Opcode::Div(r1, r2, r3).into())
            }
            "Imm" => {
                err_from_ordering(line.0.len().cmp(&3), &line, &file_name)?;
                let register = get_reg_or_ret(1, &line, &file_name)?;

                let imm_value = match Bit13Literal::try_from(line.0[2].value.as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(CompError(line, 2, "Invalid number literal", file_name)),
                };

                buffer.push(Opcode::Imm(register, imm_value).into())
            }
            "Push" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let register = get_reg_or_ret(1, &line, &file_name)?;

                buffer.push(Opcode::Push(register).into())
            }
            "Pop" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let register = get_reg_or_ret(1, &line, &file_name)?;

                buffer.push(Opcode::Pop(register).into())
            }
            "Cmp" => {
                err_from_ordering(line.0.len().cmp(&3), &line, &file_name)?;
                let r1 = get_reg_or_ret(1, &line, &file_name)?;
                let r2 = get_reg_or_ret(2, &line, &file_name)?;

                buffer.push(Opcode::Cmp(r1, r2).into())
            }
            "Jmp" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let imm_value = match Bit13Literal::try_from(line.0[1].value.as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(CompError(line, 2, "Invalid number literal", file_name)),
                };

                buffer.push(Opcode::Jmp(imm_value).into())
            }
            "Je" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let imm_value = match Bit13Literal::try_from(line.0[1].value.as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(CompError(line, 2, "Invalid number literal", file_name)),
                };

                buffer.push(Opcode::Je(imm_value).into())
            }
            "Jne" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let imm_value = match Bit13Literal::try_from(line.0[1].value.as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(CompError(line, 2, "Invalid number literal", file_name)),
                };

                buffer.push(Opcode::Jne(imm_value).into())
            }
            "Jg" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let imm_value = match Bit13Literal::try_from(line.0[1].value.as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(CompError(line, 2, "Invalid number literal", file_name)),
                };

                buffer.push(Opcode::Jg(imm_value).into())
            }
            "Jge" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let imm_value = match Bit13Literal::try_from(line.0[1].value.as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(CompError(line, 2, "Invalid number literal", file_name)),
                };

                buffer.push(Opcode::Jge(imm_value).into())
            }
            "Jz" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let imm_value = match Bit13Literal::try_from(line.0[1].value.as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(CompError(line, 2, "Invalid number literal", file_name)),
                };

                buffer.push(Opcode::Jz(imm_value).into())
            }
            "Jnz" => {
                err_from_ordering(line.0.len().cmp(&2), &line, &file_name)?;
                let imm_value = match Bit13Literal::try_from(line.0[1].value.as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(CompError(line, 2, "Invalid number literal", file_name)),
                };

                buffer.push(Opcode::Jnz(imm_value).into())
            }
            "Syscall" => {
                err_from_ordering(line.0.len().cmp(&1), &line, &file_name)?;
                buffer.push(Opcode::Syscall.into())
            }
            _ => return Err(CompError(line, 0, "Unknown instruction", file_name)),
        }
    }
    // Exit with 0 exit code
    buffer.push(Opcode::Imm(Register::A, Bit13Literal::try_from("0").unwrap()).into());
    buffer.push(Opcode::Imm(Register::B, Bit13Literal::try_from("0").unwrap()).into());
    buffer.push(Opcode::Syscall.into());

    Ok(buffer)
}

fn write_binary_to_file(bin: Vec<u32>, file: String) -> Result<(), std::io::Error> {
    let program: String = bin
        .iter()
        .map(|num| format!("{:08x}", num)) // Format as hex, zero-padded to 8 characters
        .map(|hex_str| hex_str.chars().rev().collect::<String>()) // Reverse the hex string
        .collect::<Vec<String>>()
        .join(" ");

    let mut file = File::create(file)?;
    file.write_all(program.as_bytes())?;

    Ok(())
}

fn dissasemble_to_file(input_file: String, output: String) -> Result<(), Box<dyn Error>> {
    let program = std::fs::read_to_string(input_file)?;

    let mut instructions: Vec<Opcode> = vec![];

    for line in program.split_whitespace() {
        let mut reversed = String::with_capacity(line.len());

        for ch in line.chars().rev() {
            reversed.push(ch);
        }

        instructions.push(Opcode::from(u32::from_str_radix(&reversed, 16)?));
    }

    let mut output = std::fs::File::create(output)?;

    for ins in instructions {
        output.write_all(format!("{}\n", ins).as_bytes())?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let source = std::fs::read_to_string(&args.input_file)?;

    if args.dissasemble {
        dissasemble_to_file(args.input_file, args.output_file)?;
    } else {
        let program = match assemble(args.input_file, source) {
            Ok(prog) => prog,
            Err(e) => {
                eprintln!("{}", e);
                return Ok(());
            }
        };
        write_binary_to_file(program, args.output_file)?;
    }

    Ok(())
}
