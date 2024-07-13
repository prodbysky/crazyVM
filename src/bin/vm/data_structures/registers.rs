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
