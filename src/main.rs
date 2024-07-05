use std::{env::args, fs};

use vm::vm::{asm_to_instructions, bytes_to_instructions, instruction_to_bytes, VM};

fn main() {
    let arguments: Vec<_> = args().collect();

    match arguments.as_slice() {
        [_, flag, file_name] => match flag.as_str() {
            // take a file, load it into memory, and then run it
            "-d" | "--decode" => {
                let file_str: Vec<u8> = fs::read(file_name).expect("Could not read");
                let instructions = bytes_to_instructions(&file_str);
                let mut vm = VM::default();
                vm.run(&instructions);
            }
            // Run the assembly file directly
            "-r" | "--run" => {
                let file_str: String = fs::read_to_string(file_name).expect("Could not read");
                let instructions = asm_to_instructions(&file_str);
                let mut vm = VM::default();
                vm.run(&instructions);
            }
            _ => unimplemented!(),
        },
        [_, flag, input_file, output_file] => match flag.as_str() {
            "-e" | "--encode" => {
                let file_str: String = fs::read_to_string(input_file).expect("Could not read");
                let instructions = asm_to_instructions(&file_str);
                let bytes = instruction_to_bytes(&instructions);
                fs::write(output_file, bytes).expect("Could not write to file");
            }
            _ => unimplemented!(),
        },
        _ => todo!(),
    }
}
