use instructions::Instruction;
use memory::Memory;
use std::env;
use std::fs;

mod instructions;
mod interpreter;
mod memory;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: chip8 <file>");
        return;
    }

    let filename = &args[1];

    let contents = fs::read(filename).expect("Error reading the given filename");

    let mut memory = Memory {
        delay: 0,
        sound: 0,
        stack_pointer: 0,
        program_counter: memory::INTERPRETER_SIZE as u16,
        i: 0,
        registers: [0; 16],
        stack: [0; 16],
        display: [[0; 64]; 32],
        ram: [0; 0xFFF],
    };

    //TODO Load sprites into interpreter area of memory

    for index in 0..contents.len() {
        memory.ram[memory::INTERPRETER_SIZE + index] = contents[index];
    }

    let mut last_address = memory.program_counter;
    loop {
        let instruction = interpreter::parse(
            memory.ram[memory.program_counter as usize],
            memory.ram[memory.program_counter as usize + 1],
        );

        if instruction == Instruction::Invalid {
            break;
        }

        interpreter::execute(&mut memory, instruction);

        if last_address == memory.program_counter {
            break;
        }

        last_address = memory.program_counter;
    }
}
