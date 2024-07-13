use crate::data_structures::{Ram, Registers, Rom};

pub struct CrazyVM {
    program: Rom,
    registers: Registers,
    memory: Ram,
}
