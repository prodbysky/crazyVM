use core::fmt;
use std::ops::{Index, IndexMut};

pub struct Rom {
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct OutOfBoundsError(usize);

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tried to access out of bounds memory @ {}", self.0)
    }
}

impl Rom {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn read(&self, index: usize) -> Result<u8, OutOfBoundsError> {
        if self.data.len() <= index {
            return Err(OutOfBoundsError(index));
        }

        Ok(self.data[index])
    }
}

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

pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn write(&mut self, byte: u8, index: usize) -> Result<(), OutOfBoundsError> {
        if index >= self.data.len() {
            return Err(OutOfBoundsError(index));
        }
        self.data[index] = byte;
        Ok(())
    }

    pub fn write_many(&mut self, bytes: &[u8], mut index: usize) -> Result<(), OutOfBoundsError> {
        if index + bytes.len() > self.data.len() {
            return Err(OutOfBoundsError(index));
        }

        for byte in bytes {
            self.write(*byte, index)?;
            index += 1;
        }

        Ok(())
    }

    pub fn read(&self, index: usize) -> Result<u8, OutOfBoundsError> {
        if self.data.len() <= index {
            return Err(OutOfBoundsError(index));
        }

        Ok(self.data[index])
    }

    pub fn read_many(&self, mut index: usize, mut n: usize) -> Result<Vec<u8>, OutOfBoundsError> {
        if self.data.len() <= index {
            return Err(OutOfBoundsError(index));
        }

        let mut buf = Vec::with_capacity(n);

        while n > 0 {
            buf.push(self.read(index)?);
            index += 1;
            n -= 1;
        }

        Ok(buf)
    }
}
