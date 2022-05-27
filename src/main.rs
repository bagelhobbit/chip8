use instructions::Instruction;
use memory::Memory;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::env;
use std::fs;
use std::time::Duration;

mod display_constants;
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

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut pressed_keys = Vec::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    pressed_keys.push(key);
                }
                _ => {}
            }
        }

        let instruction = interpreter::parse(
            memory.ram[memory.program_counter as usize],
            memory.ram[memory.program_counter as usize + 1],
        );

        if instruction == Instruction::Invalid {
            continue;
        }

        let draw = interpreter::execute(&mut memory, instruction, &mut pressed_keys);

        match draw {
            None => (),
            Some((x, y, length)) => {
                let mut filled_rects = Vec::new();
                let mut blank_rects = Vec::new();

                for row in y..(y + length as usize) {
                    for col in x..(x + 8) {
                        let row_index = row % display_constants::HEIGHT as usize;
                        let col_index = col % display_constants::WIDTH as usize;

                        if memory.display[row_index][col_index] == 1 {
                            filled_rects.push(Rect::new(
                                col_index as i32 * 10,
                                row_index as i32 * 10,
                                10,
                                10,
                            ));
                        } else {
                            blank_rects.push(Rect::new(
                                col_index as i32 * 10,
                                row_index as i32 * 10,
                                10,
                                10,
                            ));
                        }
                    }
                }

                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.fill_rects(&filled_rects).unwrap();

                canvas.set_draw_color(Color::RGB(0, 255, 255));
                canvas.fill_rects(&blank_rects).unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
