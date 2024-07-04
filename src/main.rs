use vm::instruction::Instruction::*;
use vm::register::Reg::*;
use vm::vm::bytes_to_instructions;
use vm::vm::instruction_to_bytes;
use vm::vm::VM;

fn main() {
    let instructions = vec![
        PutReg(20, R0),
        PutReg(10, R1),
        Sub(R1, R0),
        PrintReg(R0),
        Ret,
    ];

    println!("Instructions: ");
    println!("{:?}", instructions);
    println!("Encoded Instructions: ");
    let encoded = instruction_to_bytes(&instructions);
    println!("{:?}", encoded);
    let decoded = bytes_to_instructions(&encoded);
    println!("{:?}", decoded);

    let mut vm = VM::default();
    vm.run(&instructions);
}
