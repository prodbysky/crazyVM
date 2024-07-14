use crate::data_structures::registers::Register;

#[derive(Debug)]
pub struct Bit13Literal(u16);

impl From<Bit13Literal> for u32 {
    fn from(val: Bit13Literal) -> Self {
        val.0 as u32
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
    Add(Register, Register, Register),
    Imm(Register, Bit13Literal),
    Push(Register),
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value.op() {
            0x01 => Self::Add(value.r1(), value.r2(), value.r3()),
            0x07 => Self::Imm(value.r1(), value.lit13()),
            0x08 => Self::Push(value.r1()),
            _ => {
                eprintln!("Unknown opcode encountered: {}", value.op());
                unreachable!()
            }
        }
    }
}

trait Instruction {
    fn op(&self) -> u8;
    fn r1(&self) -> Register;
    fn r2(&self) -> Register;
    fn r3(&self) -> Register;
    fn lit13(&self) -> Bit13Literal;
}

impl Instruction for u32 {
    fn op(&self) -> u8 {
        (self & 0x000000ff) as u8
    }

    fn r1(&self) -> Register {
        ((self & 0x700) >> 8).into()
    }

    fn r2(&self) -> Register {
        ((self & 0x3800) >> 11).into()
    }

    fn r3(&self) -> Register {
        ((self & 0x1c000) >> 14).into()
    }

    fn lit13(&self) -> Bit13Literal {
        Bit13Literal(((self & 0x8fff00) >> 11) as u16)
    }
}
