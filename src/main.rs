use vm::instruction::Instruction::*;
use vm::register::Reg::*;
use vm::vm::bytes_to_instructions;
use vm::vm::instruction_to_bytes;
use vm::vm::VM;

fn main() {
    // This program corresponds to this higher level form:
    // let x = 20;
    // let y = 20;
    // if (x == y) {
    //   print(0);
    // } else {
    //   print(1);
    // }
    //
    // let instructions = vec![
    //     PutReg(20, R0),
    //     PutReg(20, R1),
    //     Cmp(R0, R1),
    //     JumpFalse(3),
    //     PutReg(0, R0),
    //     PrintReg(R0),
    //     Jump(2),
    //     PutReg(1, R0),
    //     PrintReg(R0),
    //     PutReg(0, R0),
    //     Ret,
    // ];

    // println!("Instructions: ");
    // println!("{:?}", instructions);
    // println!("Encoded Instructions: ");
    // let encoded = instruction_to_bytes(&instructions);
    // println!("Encoded Instructions: ");
    // println!("{:?}", encoded);
    // let decoded = bytes_to_instructions(&encoded);
    // println!("Decoded Instructions: ");
    // println!("{:?}", decoded);

    // let mut vm = VM::default();
    // vm.run(&instructions);

    // This program corresponds to this higher level form:
    // let x = [1, 2, 3];
    // print(x);
    let instructions = vec![
        PutReg(1, R0),
        CopyRS(R0, 0),
        PutReg(2, R0),
        CopyRS(R0, 1),
        PutReg(3, R0),
        CopyRS(R0, 2),
        CopySR(0, R0),
        PrintReg(R0),
        CopySR(1, R0),
        PrintReg(R0),
        CopySR(2, R0),
        PrintReg(R0),
        PutReg(0, R0),
        Ret,
    ];

    println!("Instructions: ");
    println!("{:?}", instructions);
    println!("Encoded Instructions: ");
    let encoded = instruction_to_bytes(&instructions);
    println!("Encoded Instructions: ");
    println!("{:?}", encoded);
    let decoded = bytes_to_instructions(&encoded);
    println!("Decoded Instructions: ");
    println!("{:?}", decoded);

    let mut vm = VM::default();
    vm.run(&instructions);
}
