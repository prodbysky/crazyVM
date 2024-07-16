use std::fmt;
use std::ops::{Index, IndexMut};

/// A, B, C, D - General purpose registers
/// SP - Stack pointer
/// PC - Program pointer,
/// Flag - Flags (over/underflow, comparisons)
/// Zero - Always zero, writing to this does nothing
/// Count - Never used by the program
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    SP,
    PC,
    Flag,
    Zero,
    A,
    B,
    C,
    D,
    Count,
}

impl From<u32> for Register {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::SP,
            1 => Self::PC,
            2 => Self::Flag,
            3 => Self::Zero,
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            7 => Self::D,
            _ => unreachable!(),
        }
    }
}

impl From<Register> for u32 {
    fn from(value: Register) -> Self {
        use Register::*;
        match value {
            SP => 0,
            PC => 1,
            Flag => 2,
            Zero => 3,
            A => 4,
            B => 5,
            C => 6,
            D => 7,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Register::*;
        match *self {
            SP => write!(f, "SP"),
            PC => write!(f, "PC"),
            Flag => write!(f, "Flag"),
            Zero => write!(f, "Zero"),
            A => write!(f, "A"),
            B => write!(f, "B"),
            C => write!(f, "C"),
            D => write!(f, "D"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct InvalidRegister;

impl TryFrom<&str> for Register {
    type Error = InvalidRegister;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SP" => Ok(Self::SP),
            "PC" => Ok(Self::PC),
            "Flag" => Ok(Self::Flag),
            "Zero" => Ok(Self::Zero),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            _ => Err(InvalidRegister),
        }
    }
}

pub struct Registers {
    registers: [u32; Register::Count as usize],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            registers: [0; Register::Count as usize],
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Register::*;
        writeln!(f, "SP: {} PC: {} Flag: {}", self[SP], self[PC], self[Flag])?;
        write!(
            f,
            "A: {}   B: {}   C: {}   D: {}",
            self[A], self[B], self[C], self[D]
        )
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<Register> for Registers {
    type Output = u32;

    fn index(&self, index: Register) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}
