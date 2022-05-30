use instructions::Instruction;
use memory::Memory;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashSet;
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
        .window(
            "CHIP8 Emulator",
            display_constants::WIDTH * display_constants::SCALE,
            display_constants::HEIGHT * display_constants::SCALE,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut old_scancodes: HashSet<Scancode> = HashSet::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let pressed_keys = pressed_keycode_set(&event_pump);
        let new_keys: HashSet<Keycode> =
            newly_pressed(&old_scancodes, &pressed_scancode_set(&event_pump))
                .iter()
                .filter_map(|&s| Keycode::from_scancode(s))
                .collect();

        for _ in 0..20 {
            let instruction = interpreter::parse(
                memory.ram[memory.program_counter as usize],
                memory.ram[memory.program_counter as usize + 1],
            );

            if instruction == Instruction::Invalid {
                continue;
            }

            interpreter::execute(&mut memory, instruction, &pressed_keys, &new_keys);
        }

        old_scancodes = pressed_scancode_set(&event_pump);

        let mut filled_rects = Vec::new();
        let mut blank_rects = Vec::new();

        for row in 0..(display_constants::HEIGHT as usize) {
            for col in 0..(display_constants::WIDTH as usize) {
                if memory.display[row][col] == 1 {
                    filled_rects.push(Rect::new(
                        col as i32 * display_constants::SCALE as i32,
                        row as i32 * display_constants::SCALE as i32,
                        display_constants::SCALE,
                        display_constants::SCALE,
                    ));
                } else {
                    blank_rects.push(Rect::new(
                        col as i32 * display_constants::SCALE as i32,
                        row as i32 * display_constants::SCALE as i32,
                        display_constants::SCALE,
                        display_constants::SCALE,
                    ));
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rects(&filled_rects).unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.fill_rects(&blank_rects).unwrap();

        if memory.delay > 0 {
            memory.delay -= 1;
        }

        if memory.sound > 0 {
            memory.sound -= 1;
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn pressed_scancode_set(event_pump: &sdl2::EventPump) -> HashSet<Scancode> {
    event_pump.keyboard_state().pressed_scancodes().collect()
}

fn pressed_keycode_set(event_pump: &sdl2::EventPump) -> HashSet<Keycode> {
    event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect()
}

fn newly_pressed(old: &HashSet<Scancode>, new: &HashSet<Scancode>) -> HashSet<Scancode> {
    new - old
    // sugar for: new.difference(old).collect()
}
