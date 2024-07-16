use core::fmt;
use std::cmp::Ordering;

use crate::registers::Register;

/// The literal available in the Imm instruction
#[derive(Debug, Clone, Copy)]
pub struct Bit13Literal(u16);

impl From<Bit13Literal> for u32 {
    fn from(val: Bit13Literal) -> Self {
        val.0 as u32
    }
}

#[derive(Debug)]
pub enum Invalid13BitLitError {
    TooBig,
    InvalidDigit,
}

impl TryFrom<&str> for Bit13Literal {
    type Error = Invalid13BitLitError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (num, base) = match value.chars().nth(0).unwrap() {
            '#' => (&value[1..], 16),
            '$' => (&value[1..], 2),
            _ => (value, 10),
        };
        let num = match u16::from_str_radix(num, base) {
            Ok(num) => num,
            Err(_) => return Err(Invalid13BitLitError::InvalidDigit),
        };

        match num.cmp(&8191) {
            Ordering::Greater => Err(Invalid13BitLitError::TooBig),
            _ => Ok(Self(num)),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// Arithmetic operations
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Mul(Register, Register, Register),
    Div(Register, Register, Register),
    /// Misc operations
    Imm(Register, Bit13Literal),
    /// Stack operations
    Push(Register),
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value.op() {
            0x01 => Self::Add(value.r1(), value.r2(), value.r3()),
            0x02 => Self::Sub(value.r1(), value.r2(), value.r3()),
            0x03 => Self::Mul(value.r1(), value.r2(), value.r3()),
            0x04 => Self::Div(value.r1(), value.r2(), value.r3()),
            0x05 => Self::Imm(value.r1(), value.lit13()),
            0x06 => Self::Push(value.r1()),
            _ => {
                eprintln!("Unknown opcode encountered: {}", value.op());
                unreachable!()
            }
        }
    }
}

impl From<Opcode> for u32 {
    fn from(val: Opcode) -> Self {
        match val {
            Opcode::Add(r1, r2, r3) => {
                let mut ins: u32 = 0x01;
                ins |= u32::from(r1) << 8;
                ins |= u32::from(r2) << 11;
                ins |= u32::from(r3) << 14;
                ins
            }
            Opcode::Sub(r1, r2, r3) => {
                let mut ins: u32 = 0x02;
                ins |= u32::from(r1) << 8;
                ins |= u32::from(r2) << 11;
                ins |= u32::from(r3) << 14;
                ins
            }
            Opcode::Mul(r1, r2, r3) => {
                let mut ins: u32 = 0x03;
                ins |= u32::from(r1) << 8;
                ins |= u32::from(r2) << 11;
                ins |= u32::from(r3) << 14;
                ins
            }
            Opcode::Div(r1, r2, r3) => {
                let mut ins: u32 = 0x04;
                ins |= u32::from(r1) << 8;
                ins |= u32::from(r2) << 11;
                ins |= u32::from(r3) << 14;
                ins
            }
            Opcode::Imm(r1, imm) => {
                let mut ins: u32 = 0x05;
                ins |= u32::from(r1) << 8;
                ins |= u32::from(imm) << 11;
                ins
            }
            Opcode::Push(r1) => {
                let mut ins: u32 = 0x06;
                ins |= u32::from(r1) << 8;
                ins
            }
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Opcode::*;
        let op_name = match *self {
            Add(..) => "Add",
            Sub(..) => "Sub",
            Mul(..) => "Mul",
            Div(..) => "Div",
            Imm(..) => "Imm",
            Push(..) => "Push",
        };

        match *self {
            Add(r1, r2, r3) | Sub(r1, r2, r3) | Mul(r1, r2, r3) | Div(r1, r2, r3) => {
                write!(f, "{} {} {} {}", op_name, r1, r2, r3)
            }
            Imm(r1, lit) => write!(f, "{} {} {}", op_name, r1, lit.0.to_string()),
            Push(r1) => write!(f, "{} {}", op_name, r1),
        }
    }
}

/// Useful trait to be used on raw u32s to get encoded values
trait Instruction {
    fn op(&self) -> u8;
    fn r1(&self) -> Register;
    fn r2(&self) -> Register;
    fn r3(&self) -> Register;
    fn lit13(&self) -> Bit13Literal;
}

impl Instruction for u32 {
    fn op(&self) -> u8 {
        (self & 0x000000ff) as u8
    }

    fn r1(&self) -> Register {
        ((self & 0x700) >> 8).into()
    }

    fn r2(&self) -> Register {
        ((self & 0x3800) >> 11).into()
    }

    fn r3(&self) -> Register {
        ((self & 0x1c000) >> 14).into()
    }

    fn lit13(&self) -> Bit13Literal {
        Bit13Literal(((self & 0x8fff00) >> 11) as u16)
    }
}