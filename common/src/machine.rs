use crate::instructions::Opcode;
use crate::registers::{Register, Registers};
use core::fmt;

use crate::data_structures::{ram::Ram, rom::Rom};

/// The virtual machine state struct itself
pub struct CrazyVM {
    program: Rom,
    registers: Registers,
    memory: Ram,
    skipping_body: bool,
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
            skipping_body: false,
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
        if (self.registers[Register::SP] + 1) as usize >= self.memory.max_size() {
            return Err(RuntimeError::StackOverflow);
        }

        self.memory
            .write(self.registers[r], self.registers[Register::SP] as usize)
            .ok()
            .ok_or(RuntimeError::MemoryWrite)?;
        self.registers[Register::SP] += 1;
        Ok(())
    }

    fn stack_pop(&mut self, r: Register) -> Result<(), RuntimeError> {
        if self.registers[Register::SP] < 1 {
            return Err(RuntimeError::StackUnderflow);
        }
        self.registers[Register::SP] -= 1;
        self.registers[r] = self
            .memory
            .read(self.registers[Register::SP] as usize)
            .ok()
            .ok_or(RuntimeError::MemoryWrite)?;

        Ok(())
    }

    fn stack_pop_internal(&mut self) -> u32 {
        self.registers[Register::SP] -= 1;
        self.memory
            .read(self.registers[Register::SP] as usize)
            .unwrap()
    }

    fn stack_push_internal(&mut self, val: u32) -> Result<(), RuntimeError> {
        if (self.registers[Register::SP] + 1) as usize >= self.memory.max_size() {
            return Err(RuntimeError::StackOverflow);
        }

        self.memory
            .write(val, self.registers[Register::SP] as usize)
            .ok()
            .ok_or(RuntimeError::MemoryWrite)?;
        self.registers[Register::SP] += 1;
        Ok(())
    }

    pub fn step(&mut self) -> Result<Option<u32>, RuntimeError> {
        let ins = self
            .get_next_instruction()
            .ok_or(RuntimeError::NoNextInstruction)?;

        if self.skipping_body {
            if let Opcode::Ret = ins {
                self.skipping_body = false;
                return Ok(None);
            }
            return Ok(None);
        }
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
            // Zero, Less, More, Eq, NotEq
            Opcode::Cmp(r1, r2) => {
                self.registers[Register::Flag] = 0;
                self.registers[Register::Flag] |= (self.registers[r1] == 0) as u32;
                self.registers[Register::Flag] |=
                    ((self.registers[r1] < self.registers[r2]) as u32) << 1;
                self.registers[Register::Flag] |=
                    ((self.registers[r1] > self.registers[r2]) as u32) << 2;
                self.registers[Register::Flag] |=
                    ((self.registers[r1] == self.registers[r2]) as u32) << 3;
                self.registers[Register::Flag] |=
                    ((self.registers[r1] != self.registers[r2]) as u32) << 4;
            }
            Opcode::Jmp(imm) => {
                self.registers[Register::PC] = imm.into();
            }
            Opcode::Je(imm) => {
                if self.registers[Register::Flag] & (1 << 3) == (1 << 3) {
                    self.registers[Register::PC] = imm.into()
                }
            }
            Opcode::Jne(imm) => {
                if self.registers[Register::Flag] & (1 << 4) == (1 << 4) {
                    self.registers[Register::PC] = imm.into()
                }
            }
            Opcode::Jg(imm) => {
                if self.registers[Register::Flag] & (1 << 2) == (1 << 2) {
                    self.registers[Register::PC] = imm.into()
                }
            }
            Opcode::Jge(imm) => {
                if self.registers[Register::Flag] & (1 << 2 | 1 << 3) == (1 << 2 | 1 << 3) {
                    self.registers[Register::PC] = imm.into()
                }
            }
            Opcode::Jz(imm) => {
                if self.registers[Register::Flag] & 1 == 1 {
                    self.registers[Register::PC] = imm.into()
                }
            }
            Opcode::Jnz(imm) => {
                if self.registers[Register::Flag] & 1 == 0 {
                    self.registers[Register::PC] = imm.into()
                }
            }
            Opcode::Jl(imm) => {
                if self.registers[Register::Flag] & (1 << 1) == (1 << 1) {
                    self.registers[Register::PC] = imm.into()
                }
            }
            Opcode::Jle(imm) => {
                if self.registers[Register::Flag] & (1 << 1 | 1 << 3) == (1 << 1 | 1 << 3) {
                    self.registers[Register::PC] = imm.into()
                }
            }
            Opcode::Ret => {
                self.stack_pop(Register::PC).unwrap();
            }
            Opcode::Call(imm) => {
                self.stack_push(Register::PC).unwrap();
                self.registers[Register::PC] = (imm.0 + 1) as u32;
            }
            Opcode::Fn => {
                self.skipping_body = true;
            }
            Opcode::StackAdd => {
                let a = self.stack_pop_internal();
                let b = self.stack_pop_internal();
                self.stack_push_internal(b + a).unwrap();
            }
            Opcode::StackSub => {
                let a = self.stack_pop_internal();
                let b = self.stack_pop_internal();
                self.stack_push_internal(b - a).unwrap();
            }
            Opcode::StackMul => {
                let a = self.stack_pop_internal();
                let b = self.stack_pop_internal();
                self.stack_push_internal(b * a).unwrap();
            }
            Opcode::StackDiv => {
                let a = self.stack_pop_internal();
                let b = self.stack_pop_internal();
                self.stack_push_internal(b / a).unwrap();
            }
            Opcode::Syscall => match self.registers[Register::A] {
                0 => {
                    let code = self.registers[Register::B];
                    return Ok(Some(code));
                }
                1 => {
                    let _fd = self.registers[Register::B];
                    let base_addr = self.registers[Register::C];
                    let len = self.registers[Register::D];
                    let saved_addr = self.registers[Register::SP];
                    self.registers[Register::SP] = base_addr;
                    let mut buf = String::new();

                    let stdin = std::io::stdin();
                    stdin.read_line(&mut buf).unwrap();

                    for (i, c) in buf.bytes().enumerate() {
                        if i >= len as usize {
                            break;
                        }
                        self.registers[Register::A] = c as u32;
                        self.stack_push(Register::A).unwrap();
                    }
                    self.registers[Register::SP] = saved_addr;
                }
                2 => {
                    let _fd = self.registers[Register::B];
                    let base_addr = self.registers[Register::C];
                    let len = self.registers[Register::D];
                    let saved_addr = self.registers[Register::SP];
                    self.registers[Register::SP] = base_addr;

                    for i in base_addr..base_addr + len {
                        print!(
                            "{}",
                            char::from_u32(self.memory.read(i as usize).unwrap()).unwrap()
                        );
                    }
                    self.registers[Register::SP] = saved_addr;
                }
                _ => {
                    todo!()
                }
            },
        }

        Ok(None)
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
