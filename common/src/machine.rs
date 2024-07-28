use core::fmt;

use crate::instructions::Opcode;
use crate::registers::{Register, Registers};

use crate::data_structures::{ram::Ram, rom::Rom};

/// The virtual machine state struct itself
pub struct CrazyVM {
    program: Rom,
    registers: Registers,
    memory: Ram,
}

/// NoNextInstruction - Signals to the manager to stop stepping the VM
#[derive(Debug)]
pub enum RuntimeError {
    StackOverflow,
    StackUnderflow,
    MemoryWrite,
    NoNextInstruction,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            RuntimeError::MemoryWrite => "Failed to write to memory!",
            RuntimeError::StackOverflow => "Stack overflew!",
            RuntimeError::StackUnderflow => "Stack underflew!",
            RuntimeError::NoNextInstruction => "Failed to get next instruction!",
        };

        write!(f, "{}", msg)
    }
}

impl CrazyVM {
    pub fn new(program: &[u32], mem_size: usize) -> Self {
        Self {
            program: program.into(),
            registers: Default::default(),
            memory: Ram::new(mem_size),
        }
    }

    fn get_next_instruction(&mut self) -> Option<Opcode> {
        match self.program.read(self.registers[Register::PC] as usize) {
            Ok(p) => {
                self.registers[Register::PC] += 1;
                Some(p.into())
            }
            Err(_) => None,
        }
    }

    fn stack_push(&mut self, r: Register) -> Result<(), RuntimeError> {
        if (self.registers[Register::SP] + 4) as usize >= self.memory.max_size() {
            return Err(RuntimeError::StackOverflow);
        }

        self.memory
            .write(self.registers[r], self.registers[Register::SP] as usize)
            .ok()
            .ok_or(RuntimeError::MemoryWrite)?;
        self.registers[Register::SP] += 4;
        Ok(())
    }

    fn stack_pop(&mut self, r: Register) -> Result<(), RuntimeError> {
        if self.registers[Register::SP] < 4 {
            return Err(RuntimeError::StackUnderflow);
        }
        self.registers[Register::SP] -= 4;
        self.registers[r] = self
            .memory
            .read(self.registers[Register::SP] as usize)
            .ok()
            .ok_or(RuntimeError::MemoryWrite)?;

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        let ins = self
            .get_next_instruction()
            .ok_or(RuntimeError::NoNextInstruction)?;

        match ins {
            Opcode::Add(r1, r2, r3) => {
                self.registers[r3] = self.registers[r1] + self.registers[r2];
            }
            Opcode::Sub(r1, r2, r3) => {
                self.registers[r3] = self.registers[r1] - self.registers[r2];
            }
            Opcode::Mul(r1, r2, r3) => {
                self.registers[r3] = self.registers[r1] * self.registers[r2];
            }
            Opcode::Div(r1, r2, r3) => {
                self.registers[r3] = self.registers[r1] / self.registers[r2];
            }
            Opcode::Imm(r1, imm) => {
                self.registers[r1] = imm.into();
            }
            Opcode::Push(r1) => self.stack_push(r1)?,
            Opcode::Pop(r1) => self.stack_pop(r1)?,
            Opcode::JumpAbs(r1) => {
                self.registers[Register::PC] = self.registers[r1];
            }
            Opcode::JumpRel(r1) => {
                self.registers[Register::PC] += self.registers[r1];
            }
        }

        Ok(())
    }

    /// Used for debug purposes
    pub fn dump_state(&self) {
        eprintln!("{}", self.registers);

        if self.registers[Register::SP] == 0 {
            return;
        }
        eprintln!("Stack:");
        for i in 0..self.registers[Register::SP] {
            eprintln!("{}", self.memory.read(i as usize).unwrap());
        }
    }
}
