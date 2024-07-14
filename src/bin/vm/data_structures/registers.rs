use core::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
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
