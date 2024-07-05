use vm::instruction::Instruction::*;
use vm::register::Reg::*;
use vm::vm::bytes_to_instructions;
use vm::vm::VM;
use vm::vm::{instruction_to_bytes, instructions_to_asm};

fn main() {
    let instructions = vec![
        PutReg(0, R0),
        PutReg(1, R1),
        PutReg(5, R2),
        Cmp(R0, R2),
        JumpTrue(3),
        PrintReg(R0),
        Add(R1, R0),
        Jump(-5),
        PutReg(0, R0),
        Ret,
    ];

    println!("Instructions: ");
    println!("{:?}", instructions);
    println!("Encoded Instructions: ");
    let encoded = instruction_to_bytes(&instructions);
    println!("Encoded Instructions: ");
    println!("{:?}", encoded);
    let asm = instructions_to_asm(&instructions);
    println!("assembly instructions : ");
    println!("{:?}", asm);
    let decoded = bytes_to_instructions(&encoded);
    println!("Decoded Instructions: ");
    println!("{:?}", decoded);

    let mut vm = VM::default();
    vm.run(&instructions);
}
