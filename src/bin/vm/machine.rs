use crate::data_structures::{ram::Ram, registers::Registers, rom::Rom};

pub struct CrazyVM {
    program: Rom,
    registers: Registers,
    memory: Ram,
}
