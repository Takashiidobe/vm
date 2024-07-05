use crate::{
    instruction::Instruction,
    utils::{u8_to_i16, u8_to_u16, REGISTER_COUNT, STACK_SIZE},
};

use std::process::exit;
use Instruction::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    registers: [u16; REGISTER_COUNT],
    stack: [u16; STACK_SIZE],
    ip: usize,
    instructions: Vec<Instruction>,
    cond: bool,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            stack: [0; 65536],
            registers: Default::default(),
            ip: 0,
            instructions: Default::default(),
            cond: false,
        }
    }
}

impl VM {
    pub fn run(&mut self, instructions: &[Instruction]) {
        while self.ip < instructions.len() {
            let instruction = &instructions[self.ip];
            self.run_instruction(instruction);
            self.ip += 1;
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            PrintReg(reg) => println!("{}", self.registers[*reg as usize]),
            Add(r1, r2) => self.registers[*r2 as usize] += self.registers[*r1 as usize],
            Sub(r1, r2) => self.registers[*r2 as usize] -= self.registers[*r1 as usize],
            Mul(r1, r2) => self.registers[*r2 as usize] *= self.registers[*r1 as usize],
            Div(r1, r2) => self.registers[*r2 as usize] /= self.registers[*r1 as usize],
            Ret => exit(self.registers[0].into()),
            PutReg(num, reg) => self.registers[*reg as usize] = *num,
            CopySR(stack_pos, reg) => {
                self.registers[*reg as usize] = self.stack[*stack_pos as usize]
            }
            CopyRR(r1, r2) => self.registers[*r2 as usize] = self.registers[*r1 as usize],
            CopyRS(reg, stack_pos) => {
                self.stack[*stack_pos as usize] = self.registers[*reg as usize]
            }
            Jump(offset) => {
                if *offset > 0 {
                    self.ip += *offset as usize;
                } else {
                    self.ip -= offset.unsigned_abs() as usize;
                }
            }
            JumpTrue(offset) => {
                if self.cond {
                    if *offset > 0 {
                        self.ip += *offset as usize;
                    } else {
                        self.ip -= offset.unsigned_abs() as usize;
                    }
                }
            }
            JumpFalse(offset) => {
                if !self.cond {
                    if *offset > 0 {
                        self.ip += *offset as usize;
                    } else {
                        self.ip -= offset.unsigned_abs() as usize;
                    }
                }
            }
            Eq(r1, r2) => {
                self.cond = self.registers[*r1 as usize] == self.registers[*r2 as usize];
            }
            Neq(r1, r2) => {
                self.cond = self.registers[*r1 as usize] != self.registers[*r2 as usize];
            }
            Lt(r1, r2) => {
                self.cond = self.registers[*r1 as usize] < self.registers[*r2 as usize];
            }
            Lte(r1, r2) => {
                self.cond = self.registers[*r1 as usize] <= self.registers[*r2 as usize];
            }
            Gt(r1, r2) => {
                self.cond = self.registers[*r1 as usize] > self.registers[*r2 as usize];
            }
            Gte(r1, r2) => {
                self.cond = self.registers[*r1 as usize] >= self.registers[*r2 as usize];
            }
        }
    }
}

pub fn instruction_to_bytes(instructions: &[Instruction]) -> Vec<u8> {
    let mut bytes = vec![];
    for instruction in instructions {
        let encoded = Instruction::encode(instruction);
        bytes.extend(encoded);
    }
    bytes
}

pub fn instructions_to_asm(instructions: &[Instruction]) -> Vec<String> {
    let mut asm = vec![];
    for instruction in instructions {
        asm.push(instruction.to_string());
    }
    asm
}

fn str_to_u16(s: &str) -> u16 {
    s.parse().expect("Could not parse value to u16: {s}")
}

fn str_to_i16(s: &str) -> i16 {
    s.parse().expect("Could not parse value to i16: {s}")
}

pub fn asm_to_instructions(s: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in s.lines() {
        let l = line.trim();
        let parts: Vec<_> = l.split_whitespace().collect();
        match parts.as_slice() {
            ["#", ..] => {} // ignore comments, these start with #
            ["ret"] => instructions.push(Ret),
            ["putreg", imm, reg] => {
                instructions.push(PutReg(str_to_u16(imm), reg.to_owned().into()));
            }
            ["copysr", stack_pos, reg] => {
                instructions.push(CopySR(str_to_u16(stack_pos), reg.to_owned().into()));
            }
            ["copyrr", r1, r2] => {
                instructions.push(CopyRR(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["copyrs", stack_pos, reg] => {
                instructions.push(CopyRS(reg.to_owned().into(), str_to_u16(stack_pos)));
            }
            ["add", r1, r2] => {
                instructions.push(Add(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["sub", r1, r2] => {
                instructions.push(Sub(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["mul", r1, r2] => {
                instructions.push(Mul(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["div", r1, r2] => {
                instructions.push(Div(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["printreg", reg] => {
                instructions.push(PrintReg(reg.to_owned().into()));
            }
            ["jump", offset] => {
                instructions.push(Jump(str_to_i16(offset)));
            }
            ["jumptrue", offset] => {
                instructions.push(JumpTrue(str_to_i16(offset)));
            }
            ["jumpfalse", offset] => {
                instructions.push(JumpFalse(str_to_i16(offset)));
            }
            ["eq", r1, r2] => {
                instructions.push(Eq(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["neq", r1, r2] => {
                instructions.push(Neq(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["lt", r1, r2] => {
                instructions.push(Lt(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["lte", r1, r2] => {
                instructions.push(Lte(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["gt", r1, r2] => {
                instructions.push(Gt(r1.to_owned().into(), r2.to_owned().into()));
            }
            ["gte", r1, r2] => {
                instructions.push(Gte(r1.to_owned().into(), r2.to_owned().into()));
            }
            _ => panic!("Invalid instruction: {l}"),
        }
    }

    instructions
}

pub fn bytes_to_instructions(bytes: &[u8]) -> Vec<Instruction> {
    let mut i = 0;
    let mut instructions = vec![];

    while i < bytes.len() {
        let byte = bytes[i];
        match byte {
            0x00 => {
                instructions.push(Ret);
                i += 1;
            }
            0x01 => {
                let b1 = u8_to_u16(bytes[i + 1], bytes[i + 2]);
                instructions.push(PutReg(b1, bytes[i + 3].into()));
                i += 4;
            }
            0x02 => {
                let b1 = u8_to_u16(bytes[i + 1], bytes[i + 2]);
                instructions.push(CopySR(b1, bytes[i + 3].into()));
                i += 4;
            }
            0x03 => {
                instructions.push(CopyRR(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x04 => {
                let b1 = u8_to_u16(bytes[i + 2], bytes[i + 3]);
                instructions.push(CopyRS(bytes[i + 1].into(), b1));
                i += 4;
            }
            0x05 => {
                instructions.push(Add(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x06 => {
                instructions.push(Sub(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x07 => {
                instructions.push(Mul(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x08 => {
                instructions.push(Div(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x09 => {
                instructions.push(PrintReg(bytes[i + 1].into()));
                i += 2;
            }
            0x10 => {
                let offset = u8_to_i16(bytes[i + 1], bytes[i + 2]);
                instructions.push(Jump(offset));
                i += 3;
            }
            0x11 => {
                let offset = u8_to_i16(bytes[i + 1], bytes[i + 2]);
                instructions.push(JumpTrue(offset));
                i += 3;
            }
            0x12 => {
                let offset = u8_to_i16(bytes[i + 1], bytes[i + 2]);
                instructions.push(JumpFalse(offset));
                i += 3;
            }
            0x13 => {
                instructions.push(Eq(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x14 => {
                instructions.push(Neq(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x15 => {
                instructions.push(Lt(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x16 => {
                instructions.push(Lte(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x17 => {
                instructions.push(Gt(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x18 => {
                instructions.push(Gte(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            _ => panic!("invalid byte: {byte}"),
        }
    }

    instructions
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::Instruction,
        vm::{bytes_to_instructions, instruction_to_bytes, VM},
    };
    use quickcheck::Gen;
    use quickcheck_macros::quickcheck;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };
    use Instruction::*;

    impl Distribution<Instruction> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Instruction {
            match rng.gen_range(0..=9) {
                0x00 => Ret,
                0x01 => Sub(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x02 => Mul(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x03 => Div(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x04 => CopyRS(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u16::MAX),
                ),
                0x05 => CopyRR(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x06 => CopySR(
                    rng.gen_range(0..=u16::MAX),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x07 => PrintReg(rng.gen_range(0..=u8::MAX).into()),
                0x08 => PutReg(
                    rng.gen_range(0..=u16::MAX),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x09 => Add(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x10 => Jump(rng.gen_range(0..=i16::MAX)),
                0x11 => JumpTrue(rng.gen_range(0..=i16::MAX)),
                0x12 => JumpFalse(rng.gen_range(0..=i16::MAX)),
                0x13 => Eq(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x14 => Neq(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x15 => Lt(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x16 => Lte(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x17 => Gt(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                0x18 => Gte(
                    rng.gen_range(0..=u8::MAX).into(),
                    rng.gen_range(0..=u8::MAX).into(),
                ),
                _ => unreachable!(),
            }
        }
    }

    impl quickcheck::Arbitrary for Instruction {
        fn arbitrary(g: &mut Gen) -> Self {
            match g.choose(&[Instruction::Ret]) {
                Some(instruction) => instruction.clone(),
                _ => unreachable!(),
            }
        }
    }

    #[quickcheck]
    fn encoded_decoded_instructions(instructions: Vec<Instruction>) -> bool {
        let encoded = instruction_to_bytes(&instructions);
        let decoded = bytes_to_instructions(&encoded);
        decoded == instructions
    }

    #[quickcheck]
    fn vm_doesnt_crash(instructions: Vec<Instruction>) -> bool {
        let mut vm = VM::default();
        vm.run(&instructions);
        true
    }
}
