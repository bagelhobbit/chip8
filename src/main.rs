use crossterm::event::{poll, read, Event};
use crossterm::{
    cursor, execute, style,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use instructions::Instruction;
use memory::Memory;
use std::env;
use std::fs;
use std::io::stdout;
use std::time::Duration;

mod display_constants;
mod instructions;
mod interpreter;
mod memory;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: chip8 <file>")
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

    memory.ram[0..5].clone_from_slice(&display_constants::ZERO);
    memory.ram[5..10].clone_from_slice(&display_constants::ONE);
    memory.ram[10..15].clone_from_slice(&display_constants::TWO);
    memory.ram[15..20].clone_from_slice(&display_constants::THREE);
    memory.ram[20..25].clone_from_slice(&display_constants::FOUR);
    memory.ram[25..30].clone_from_slice(&display_constants::FIVE);
    memory.ram[30..35].clone_from_slice(&display_constants::SIX);
    memory.ram[35..40].clone_from_slice(&display_constants::SEVEN);
    memory.ram[40..45].clone_from_slice(&display_constants::EIGHT);
    memory.ram[45..50].clone_from_slice(&display_constants::NINE);
    memory.ram[50..55].clone_from_slice(&display_constants::A);
    memory.ram[55..60].clone_from_slice(&display_constants::B);
    memory.ram[60..65].clone_from_slice(&display_constants::C);
    memory.ram[65..70].clone_from_slice(&display_constants::D);
    memory.ram[70..75].clone_from_slice(&display_constants::E);
    memory.ram[75..80].clone_from_slice(&display_constants::F);

    memory.ram[memory::INTERPRETER_SIZE..(contents.len() + memory::INTERPRETER_SIZE)]
        .clone_from_slice(&contents[..]);

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        style::Print("Press any key to exit")
    )?;

    loop {
        let instruction = interpreter::parse(
            memory.ram[memory.program_counter as usize],
            memory.ram[memory.program_counter as usize + 1],
        );

        if instruction == Instruction::Invalid {
            break;
        }

        interpreter::execute(&mut memory, instruction);

        if poll(Duration::from_millis(16))? {
            if let Event::Key(event) = read()? {
                execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
                println!("{:?}", event);
                break;
            }
        }
    }

    Ok(())
}
