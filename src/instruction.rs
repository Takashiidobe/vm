use core::fmt;

use crate::{
    register::Reg,
    utils::{i16_to_u8, u16_to_u8},
};
use Instruction::*;

pub type Immediate = u16;
pub type StackPos = u16;
pub type Offset = i16;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Ret,                    // Return R0
    PutReg(Immediate, Reg), // Put u16 -> Reg
    CopySR(StackPos, Reg),  // Load Stack -> Reg
    CopyRR(Reg, Reg),       // Copy Reg -> Reg
    CopyRS(Reg, StackPos),  // Copy Reg -> Stack
    Add(Reg, Reg),          // Add R1, R2 -> R2
    Sub(Reg, Reg),          // Sub R1, R2 -> R2
    Mul(Reg, Reg),          // Mul R1, R2 -> R2
    Div(Reg, Reg),          // Div R1, R2 -> R2
    PrintReg(Reg),          // Print Reg
    Jump(Offset),           // Jump Forward or backward
    JumpTrue(Offset),       // Jump Forward or backwards if the condition flag is true.
    JumpFalse(Offset),      // Jump Forward or backwards if the condition flag is false.
    Eq(Reg, Reg),           // Compare R1 to R2, setting the condition flag to R1 == R2
    Neq(Reg, Reg),          // Compare R1 to R2, setting the condition flag to R1 != R2
    Lt(Reg, Reg),           // Compare R1 to R2, setting the condition flag to R1 < R2
    Lte(Reg, Reg),          // Compare R1 to R2, setting the condition flag to R1 <= R2
    Gt(Reg, Reg),           // Compare R1 to R2, setting the condition flag to R1 > R2
    Gte(Reg, Reg),          // Compare R1 to R2, setting the condition flag to R1 >= R2
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ret => "ret",
            PutReg(imm, reg) => &format!("putreg {imm} {reg}"),
            CopySR(pos, reg) => &format!("copysr {pos} {reg}"),
            CopyRR(r1, r2) => &format!("copyrr {r1} {r2}"),
            CopyRS(reg, pos) => &format!("copyrs {reg} {pos}"),
            Add(r1, r2) => &format!("add {r1} {r2}"),
            Sub(r1, r2) => &format!("sub {r1} {r2}"),
            Mul(r1, r2) => &format!("mul {r1} {r2}"),
            Div(r1, r2) => &format!("div {r1} {r2}"),
            PrintReg(reg) => &format!("printreg {reg}"),
            Jump(offset) => &format!("jump {offset}"),
            JumpTrue(offset) => &format!("jumptrue {offset}"),
            JumpFalse(offset) => &format!("jumpfalse {offset}"),
            Eq(r1, r2) => &format!("eq {r1} {r2}"),
            Neq(r1, r2) => &format!("neq {r1} {r2}"),
            Lt(r1, r2) => &format!("lt {r1} {r2}"),
            Lte(r1, r2) => &format!("lte {r1} {r2}"),
            Gt(r1, r2) => &format!("gt {r1} {r2}"),
            Gte(r1, r2) => &format!("gte {r1} {r2}"),
        };
        f.write_str(s)
    }
}

impl Instruction {
    pub fn encode(&self) -> Vec<u8> {
        match self.clone() {
            Ret => vec![0x00],
            PutReg(imm, reg) => {
                let [b1, b2] = u16_to_u8(imm);
                vec![0x01, b1, b2, reg as u8]
            }
            CopySR(stack_pos, reg) => {
                let [b1, b2] = u16_to_u8(stack_pos);
                vec![0x02, b1, b2, reg as u8]
            }
            CopyRR(r1, r2) => vec![0x03, r1 as u8, r2 as u8],
            CopyRS(reg, stack_pos) => {
                let [b1, b2] = u16_to_u8(stack_pos);
                vec![0x04, reg as u8, b1, b2]
            }
            Add(r1, r2) => vec![0x05, r1 as u8, r2 as u8],
            Sub(r1, r2) => vec![0x06, r1 as u8, r2 as u8],
            Mul(r1, r2) => vec![0x07, r1 as u8, r2 as u8],
            Div(r1, r2) => vec![0x08, r1 as u8, r2 as u8],
            PrintReg(reg) => vec![0x09, reg as u8],
            Jump(offset) => {
                let [b1, b2] = i16_to_u8(offset);
                vec![0x10, b1, b2]
            }
            JumpTrue(offset) => {
                let [b1, b2] = i16_to_u8(offset);
                vec![0x11, b1, b2]
            }
            JumpFalse(offset) => {
                let [b1, b2] = i16_to_u8(offset);
                vec![0x12, b1, b2]
            }
            Eq(r1, r2) => vec![0x13, r1 as u8, r2 as u8],
            Neq(r1, r2) => vec![0x14, r1 as u8, r2 as u8],
            Lt(r1, r2) => vec![0x15, r1 as u8, r2 as u8],
            Lte(r1, r2) => vec![0x16, r1 as u8, r2 as u8],
            Gt(r1, r2) => vec![0x17, r1 as u8, r2 as u8],
            Gte(r1, r2) => vec![0x18, r1 as u8, r2 as u8],
        }
    }
}
