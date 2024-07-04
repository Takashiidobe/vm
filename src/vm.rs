use crate::{
    instruction::Instruction,
    utils::{u8_to_u16, REGISTER_COUNT, STACK_SIZE},
};

use std::process::exit;
use Instruction::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    registers: [u16; REGISTER_COUNT],
    stack: [u16; STACK_SIZE],
    ip: usize,
    instructions: Vec<Instruction>,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            stack: [0; 65536],
            registers: Default::default(),
            ip: 0,
            instructions: Default::default(),
        }
    }
}

impl VM {
    pub fn run(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.run_instruction(instruction);
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
        }
    }
}

pub fn instruction_to_bytes(instructions: &[Instruction]) -> Vec<u8> {
    let mut bytes = vec![];
    for instruction in instructions {
        let encoded_instruction = Instruction::encode(instruction);
        bytes.extend(encoded_instruction);
    }
    bytes
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
                let reg = bytes[i + 3];
                instructions.push(PutReg(b1, reg.into()));
                i += 4;
            }
            0x02 => {
                let b1 = u8_to_u16(bytes[i + 1], bytes[i + 2]);
                let reg = bytes[i + 3];
                instructions.push(CopySR(b1, reg.into()));
                i += 4;
            }
            0x03 => {
                instructions.push(CopyRR(bytes[i + 1].into(), bytes[i + 2].into()));
                i += 3;
            }
            0x04 => {
                let reg = bytes[i + 1];
                let b1 = u8_to_u16(bytes[i + 2], bytes[i + 3]);
                instructions.push(CopyRS(reg.into(), b1));
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
