// 00000000 00000000 00000000 00000000

// Add:
// 10000000 xxx yyy zzz 0000000 00000000
//          r 1 r 2 r 3

// Sub:
// 01000000 xxx yyy zzz 0000000 00000000
//          r 1 r 2 r 3

// Mul:
// 11000000 xxx yyy zzz 0000000 00000000
//          r 1 r 2 r 3

// Div:
// 00100000 xxx yyy zzz 0000000 00000000
//          r 1 r 2 r 3

// Mod:
// 10100000 xxx yyy zzz 0000000 00000000
//          r 1 r 2 r 3

// Immediate value
// 11100000 xxx yyyyyyyyyyyyy
//          r 1 literal

// Push
// 00010000 xxx 00000 00000000
//          r 1

use crate::data_structures::registers::Register;

pub struct Bit13Literal;

#[repr(u8)]
pub enum Opcode {
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Mul(Register, Register, Register),
    Div(Register, Register, Register),
    Mod(Register, Register, Register),
    Imm(Register, Bit13Literal),
    Push(Register),
}
