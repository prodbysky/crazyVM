use std::fmt;
use std::ops::{Index, IndexMut};

use macros::RegisterTraits;

/// A, B, C, D - General purpose registers
/// SP - Stack pointer
/// PC - Program pointer,
/// Flag - Flags (over/underflow, comparisons)
/// Zero - Always zero, writing to this does nothing
/// Count - Never used by the program
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, RegisterTraits)]
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
        writeln!(
            f,
            "SP: {} PC: {} Flag: {:032b}",
            self[SP], self[PC], self[Flag]
        )?;
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
