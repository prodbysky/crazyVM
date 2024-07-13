use crate::data_structures::{
    ram::Ram,
    registers::{Register, Registers},
    rom::Rom,
};

pub struct CrazyVM {
    program: Rom,
    registers: Registers,
    memory: Ram,
}

impl CrazyVM {
    pub fn new(program: &[u8], mem_size: usize) -> Self {
        Self {
            program: program.into(),
            registers: Default::default(),
            memory: Ram::new(mem_size),
        }
    }

    // [11 22 33 44]

    fn get_next_instruction(&mut self) -> Option<u32> {
        let p = match self
            .program
            .read_many(self.registers[Register::PC] as usize, 4)
        {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to get next instruction: {}", e);
                return None;
            }
        };

        Some((p[3] as u32) | (p[2] as u32) << 8 | (p[1] as u32) << 16 | (p[0] as u32) << 24)
    }

    pub fn step(&mut self) {
        eprintln!("{:#04x}", self.get_next_instruction().unwrap())
    }
}
