use core::fmt;
use std::cmp::Ordering;

use macros::OpcodeTraits;

use crate::registers::Register;

/// The literal available in the Imm instruction
#[derive(Debug, Clone, Copy)]
pub struct Bit13Literal(pub u16);

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
#[derive(Debug, Clone, Copy, OpcodeTraits)]
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
    Pop(Register),
    StackAdd,
    StackSub,
    StackMul,
    StackDiv,
    /// Logical operation
    Cmp(Register, Register),

    Jmp(Bit13Literal),
    Je(Bit13Literal),
    Jne(Bit13Literal),
    Jg(Bit13Literal),
    Jge(Bit13Literal),
    Jl(Bit13Literal),
    Jle(Bit13Literal),
    Jz(Bit13Literal),
    Jnz(Bit13Literal),
    Ret,
    Call(Bit13Literal),
    Fn,

    Syscall,
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
            Pop(..) => "Pop",
            Cmp(..) => "Cmp",
            Jmp(..) => "Jmp",
            Je(..) => "Je",
            Jne(..) => "Jne",
            Jg(..) => "Jg",
            Jge(..) => "Jge",
            Jz(..) => "Jz",
            Jnz(..) => "Jnz",
            Jl(..) => "Jl",
            Jle(..) => "Jle",
            Syscall => "Syscall",
            Ret => "Ret",
            Call(..) => "Call",
            Fn => "Fn",
            StackAdd => "StackAdd",
            StackSub => "StackSub",
            StackMul => "StackMul",
            StackDiv => "StackDiv",
        };

        match *self {
            Add(r1, r2, r3) | Sub(r1, r2, r3) | Mul(r1, r2, r3) | Div(r1, r2, r3) => {
                write!(f, "{} {} {} {}", op_name, r1, r2, r3)
            }
            Cmp(r1, r2) => {
                write!(f, "{} {} {}", op_name, r1, r2)
            }
            Jmp(imm) | Je(imm) | Jne(imm) | Jg(imm) | Jge(imm) | Jz(imm) | Jnz(imm) | Jl(imm)
            | Jle(imm) | Call(imm) => {
                write!(f, "{} {}", op_name, imm.0)
            }
            Imm(r1, lit) => write!(f, "{} {} {}", op_name, r1, lit.0),
            Push(r1) | Pop(r1) => write!(f, "{} {}", op_name, r1),

            Syscall | StackAdd | StackSub | StackMul | StackDiv | Ret | Fn => {
                write!(f, "{}", op_name)
            }
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

    fn reg_1_instruction(&mut self, r1: Register) -> u32;
    fn reg_2_instruction(&mut self, r1: Register, r2: Register) -> u32;
    fn reg_3_instruction(&mut self, r1: Register, r2: Register, r3: Register) -> u32;
    fn imm_instruction(&mut self, r1: Register, imm: Bit13Literal) -> u32;
    fn jump_instruction(&mut self, imm: Bit13Literal) -> u32;
}

impl Instruction for u32 {
    fn op(&self) -> u8 {
        (self & 0x000000ff) as u8
    }

    fn r1(&self) -> Register {
        ((self >> 8) & 0x07).into()
    }

    fn r2(&self) -> Register {
        ((self >> 11) & 0x07).into()
    }

    fn r3(&self) -> Register {
        ((self >> 14) & 0x07).into()
    }

    fn lit13(&self) -> Bit13Literal {
        Bit13Literal(((self >> 11) & 0x1fff) as u16)
    }

    fn reg_1_instruction(&mut self, r1: Register) -> u32 {
        *self |= (u32::from(r1) & 0x07) << 8;
        *self
    }

    fn reg_2_instruction(&mut self, r1: Register, r2: Register) -> u32 {
        *self |= (u32::from(r1) & 0x07) << 8;
        *self |= (u32::from(r2) & 0x07) << 11;
        *self
    }

    fn reg_3_instruction(&mut self, r1: Register, r2: Register, r3: Register) -> u32 {
        *self |= (u32::from(r1) & 0x07) << 8;
        *self |= (u32::from(r2) & 0x07) << 11;
        *self |= (u32::from(r3) & 0x07) << 14;
        *self
    }

    fn imm_instruction(&mut self, r1: Register, imm: Bit13Literal) -> u32 {
        *self |= (u32::from(r1) & 0x07) << 8;
        *self |= (u32::from(imm) & 0x1fff) << 11;
        *self
    }

    fn jump_instruction(&mut self, imm: Bit13Literal) -> u32 {
        *self |= (u32::from(imm) & 0x1fff) << 11;
        *self
    }
}
