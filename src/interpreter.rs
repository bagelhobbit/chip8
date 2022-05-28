use crate::display_constants;
use crate::instructions::Address;
use crate::instructions::Instruction;
use crate::memory::Memory;
use rand::Rng;
use sdl2::keyboard::Keycode;

const KEY_MAP: [Keycode; 16] = [
    Keycode::X,
    Keycode::Num1,
    Keycode::Num2,
    Keycode::Num3,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::Z,
    Keycode::C,
    Keycode::Num4,
    Keycode::R,
    Keycode::F,
    Keycode::V,
];

pub fn parse(high_byte: u8, low_byte: u8) -> Instruction {
    match (
        get_upper_bits(high_byte),
        get_lower_bits(high_byte),
        get_upper_bits(low_byte),
        get_lower_bits(low_byte),
    ) {
        //00E0
        (0x0, 0x0, 0xE, 0x0) => Instruction::Clear,
        //00EE
        (0x0, 0x0, 0xE, 0xE) => Instruction::Return,
        //1nnn
        (0x1, high, middle, low) => Instruction::JumpTo(Address { high, middle, low }),
        //2nnn
        (0x2, high, middle, low) => Instruction::Call(Address { high, middle, low }),
        //3xkk
        (0x3, reg, _, _) => Instruction::SkipIfEqualByte {
            vx: reg as usize,
            byte: low_byte,
        },
        //4xkk
        (0x4, reg, _, _) => Instruction::SkipIfNotEqualByte {
            vx: reg as usize,
            byte: low_byte,
        },
        //5xy0
        (0x5, reg_x, reg_y, 0) => Instruction::SkipIfEqualReg {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //6xkk
        (0x6, reg, _, _) => Instruction::LoadByte {
            vx: reg as usize,
            byte: low_byte,
        },
        //7xkk
        (0x7, reg, _, _) => Instruction::AddByte {
            vx: reg as usize,
            byte: low_byte,
        },
        //8xy0
        (0x8, reg_x, reg_y, 0x0) => Instruction::LoadReg {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //8xy1
        (0x8, reg_x, reg_y, 0x1) => Instruction::Or {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //8xy2
        (0x8, reg_x, reg_y, 0x2) => Instruction::And {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //8xy3
        (0x8, reg_x, reg_y, 0x3) => Instruction::Xor {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //8xy4
        (0x8, reg_x, reg_y, 0x4) => Instruction::AddReg {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //8xy5
        (0x8, reg_x, reg_y, 0x5) => Instruction::Subtract {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //8xy6
        (0x8, reg_x, _, 0x6) => Instruction::ShiftRight { vx: reg_x as usize },
        //8xy7
        (0x8, reg_x, reg_y, 0x7) => Instruction::SubtractReverse {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //8xyE
        (0x8, reg_x, _, 0xE) => Instruction::ShiftLeft { vx: reg_x as usize },
        //9xy0
        (0x9, reg_x, reg_y, 0x0) => Instruction::SkipIfNotEqualReg {
            vx: reg_x as usize,
            vy: reg_y as usize,
        },
        //Annn
        (0xA, high, middle, low) => Instruction::LoadAddress(Address { high, middle, low }),
        //Bnnn
        (0xB, high, middle, low) => Instruction::JumpOffset(Address { high, middle, low }),
        //Cxkk
        (0xC, reg_x, _, _) => Instruction::Random {
            vx: reg_x as usize,
            byte: low_byte,
        },
        //Dxyn
        (0xD, reg_x, reg_y, nibble) => Instruction::Draw {
            vx: reg_x as usize,
            vy: reg_y as usize,
            nibble,
        },
        //Ex9E
        (0xE, reg_x, 0x9, 0xE) => Instruction::SkipIfKeyPressed { vx: reg_x as usize },
        //ExA1
        (0xE, reg_x, 0xA, 0x1) => Instruction::SkipIfNotKeyPressed { vx: reg_x as usize },
        //Fx07
        (0xF, reg_x, 0x0, 0x7) => Instruction::LoadDelay { vx: reg_x as usize },
        //Fx0A
        (0xF, reg_x, 0x0, 0xA) => Instruction::LoadKeyPressed { vx: reg_x as usize },
        //Fx15
        (0xF, reg_x, 0x1, 0x5) => Instruction::SetDelay { vx: reg_x as usize },
        //Fx18
        (0xF, reg_x, 0x1, 0x8) => Instruction::SetSound { vx: reg_x as usize },
        //Fx1E
        (0xF, reg_x, 0x1, 0xE) => Instruction::AddAddressOffset { vx: reg_x as usize },
        //Fx29
        (0xF, reg_x, 0x2, 0x9) => Instruction::LoadSprite { vx: reg_x as usize },
        //Fx33
        (0xF, reg_x, 0x3, 0x3) => Instruction::SetBCD { vx: reg_x as usize },
        //Fx55
        (0xF, reg_x, 0x5, 0x5) => Instruction::LoadRegisters { vx: reg_x as usize },
        //Fx65
        (0xF, reg_x, 0x6, 0x5) => Instruction::ReadRegisters { vx: reg_x as usize },
        _ => Instruction::Invalid,
    }
}

pub fn execute(
    memory: &mut Memory,
    instruction: Instruction,
    pressed_keys: &mut Vec<Keycode>,
) -> Option<(usize, usize, u8)> {
    // Because we read in the instruction and arguments together,
    // we need to increment the program counter by 2 normally so we don't read in the middle of anything.
    match instruction {
        Instruction::Invalid => todo!(),
        Instruction::Clear => {
            memory.display = [[0; 64]; 32];
            memory.program_counter += 2;
        }
        Instruction::Return => {
            memory.program_counter = memory.stack[memory.stack_pointer];
            memory.stack_pointer -= 1;
        }
        Instruction::JumpTo(addr) => memory.program_counter = addr.to_u16(),
        Instruction::Call(addr) => {
            memory.stack_pointer += 1;
            memory.stack[memory.stack_pointer] = memory.program_counter + 2;
            memory.program_counter = addr.to_u16();
        }
        Instruction::SkipIfEqualByte { vx, byte } => {
            if memory.registers[vx] == byte {
                memory.program_counter += 4
            } else {
                memory.program_counter += 2
            }
        }
        Instruction::SkipIfNotEqualByte { vx, byte } => {
            if memory.registers[vx] != byte {
                memory.program_counter += 4
            } else {
                memory.program_counter += 2
            }
        }
        Instruction::SkipIfEqualReg { vx, vy } => {
            if memory.registers[vx] == memory.registers[vy] {
                memory.program_counter += 4
            } else {
                memory.program_counter += 2
            }
        }
        Instruction::LoadByte { vx, byte } => {
            memory.registers[vx] = byte;
            memory.program_counter += 2;
        }
        Instruction::AddByte { vx, byte } => {
            let result = memory.registers[vx] as u16 + byte as u16;
            if result > 255 {
                memory.registers[0xF] = 1;
            }

            memory.registers[vx] = result as u8;
            memory.program_counter += 2;
        }
        Instruction::LoadReg { vx, vy } => {
            memory.registers[vx] += memory.registers[vy];
            memory.program_counter += 2;
        }
        Instruction::Or { vx, vy } => {
            memory.registers[vx] |= memory.registers[vy];
            memory.program_counter += 2;
        }
        Instruction::And { vx, vy } => {
            memory.registers[vx] &= memory.registers[vy];
            memory.program_counter += 2;
        }
        Instruction::Xor { vx, vy } => {
            memory.registers[vx] ^= memory.registers[vy];
            memory.program_counter += 2;
        }
        Instruction::AddReg { vx, vy } => {
            let result = memory.registers[vx] as u16 + memory.registers[vy] as u16;
            if result > 255 {
                memory.registers[0xF] = 1;
            }
            memory.registers[vx] = result as u8;
            memory.program_counter += 2;
        }
        Instruction::Subtract { vx, vy } => {
            if memory.registers[vx] > memory.registers[vy] {
                memory.registers[0xF] = 1;
                memory.registers[vx] -= memory.registers[vy];
            } else {
                memory.registers[0xF] = 0;
                let result = memory.registers[vx] as i16 - memory.registers[vy] as i16;
                memory.registers[vx] = result as u8;
            }
            memory.program_counter += 2;
        }
        Instruction::ShiftRight { vx } => {
            if memory.registers[vx] & 0b0000_0001 == 1 {
                memory.registers[0xF] = 1
            } else {
                memory.registers[0xF] = 0
            }
            memory.registers[vx] /= 2;
            memory.program_counter += 2;
        }
        Instruction::SubtractReverse { vx, vy } => {
            if memory.registers[vy] > memory.registers[vx] {
                memory.registers[0xF] = 1
            } else {
                memory.registers[0xF] = 0
            }
            memory.registers[vx] = memory.registers[vy] - memory.registers[vx];
            memory.program_counter += 2;
        }
        Instruction::ShiftLeft { vx } => {
            if memory.registers[vx] & 0b1000_0000 == 0b1000_0000 {
                memory.registers[0xF] = 1
            } else {
                memory.registers[0xF] = 0
            }
            memory.registers[vx] <<= 1;
            memory.program_counter += 2;
        }
        Instruction::SkipIfNotEqualReg { vx, vy } => {
            if memory.registers[vx] != memory.registers[vy] {
                memory.program_counter += 4
            } else {
                memory.program_counter += 2
            }
        }
        Instruction::LoadAddress(addr) => {
            memory.i = addr.to_u16();
            memory.program_counter += 2;
        }
        Instruction::JumpOffset(addr) => {
            memory.program_counter = addr.to_u16() + memory.registers[0] as u16;
        }
        Instruction::Random { vx, byte } => {
            let mut rng = rand::thread_rng();
            memory.registers[vx] = rng.gen_range(0..255) & byte;
            memory.program_counter += 2;
        }
        Instruction::Draw {
            vx,
            vy,
            nibble: length,
        } => {
            let x = memory.registers[vx] as usize;
            let y = memory.registers[vy] as usize;
            let mut collided = false;

            for row in 0..(length as usize) {
                let bits = get_as_bits(memory.ram[memory.i as usize + row]);

                for (bit, _) in bits.iter().enumerate() {
                    let row_index = (y + row) % display_constants::HEIGHT as usize;
                    let col_index = (x + bit) % display_constants::WIDTH as usize;

                    let old_value = memory.display[row_index][col_index];

                    memory.display[row_index][col_index] ^= bits[bit];

                    if old_value == 1 && memory.display[row_index][col_index] == 0 {
                        collided = true;
                    }
                }
            }

            if collided {
                memory.registers[0xF] = 1;
            } else {
                memory.registers[0xF] = 0;
            }

            memory.program_counter += 2;

            return Some((x, y, length));
        }
        Instruction::SkipIfKeyPressed { vx } => {
            let key_code = memory.registers[vx] as usize;

            if pressed_keys.contains(&KEY_MAP[key_code]) {
                let mut i = 0;
                while i < pressed_keys.len() {
                    if KEY_MAP[key_code] == pressed_keys[i] {
                        _ = pressed_keys.remove(i);
                    } else {
                        i += 1;
                    }
                }

                memory.program_counter += 4;
            } else {
                memory.program_counter += 2;
            }
        }
        Instruction::SkipIfNotKeyPressed { vx } => {
            let key_code = memory.registers[vx] as usize;

            if !pressed_keys.contains(&KEY_MAP[key_code]) {
                memory.program_counter += 4;
            } else {
                let mut i = 0;
                while i < pressed_keys.len() {
                    if KEY_MAP[key_code] == pressed_keys[i] {
                        _ = pressed_keys.remove(i);
                    } else {
                        i += 1;
                    }
                }

                memory.program_counter += 2;
            }
        }
        Instruction::LoadDelay { vx } => {
            memory.registers[vx] = memory.delay;
            memory.program_counter += 2;
        }
        Instruction::LoadKeyPressed { vx } => {
            let mut i = 0;
            while i < pressed_keys.len() {
                if KEY_MAP.contains(&pressed_keys[i]) {
                    memory.registers[vx] =
                        KEY_MAP.iter().position(|&k| k == pressed_keys[i]).unwrap() as u8;
                    memory.program_counter += 2;
                    break;
                } else {
                    i += 1;
                }
            }

            pressed_keys.clear();
        }
        Instruction::SetDelay { vx } => {
            memory.delay = memory.registers[vx];
            memory.program_counter += 2;
        }
        Instruction::SetSound { vx } => {
            memory.sound = memory.registers[vx];
            memory.program_counter += 2;
        }
        Instruction::AddAddressOffset { vx } => {
            memory.i += memory.registers[vx] as u16;
            memory.program_counter += 2;
        }
        Instruction::LoadSprite { vx } => {
            match memory.registers[vx] {
                0 => memory.i = 0,
                1 => memory.i = 5,
                2 => memory.i = 10,
                3 => memory.i = 15,
                4 => memory.i = 20,
                5 => memory.i = 25,
                6 => memory.i = 30,
                7 => memory.i = 35,
                8 => memory.i = 40,
                9 => memory.i = 45,
                0xA => memory.i = 50,
                0xB => memory.i = 55,
                0xC => memory.i = 60,
                0xD => memory.i = 65,
                0xE => memory.i = 70,
                0xF => memory.i = 75,
                _ => memory.i = 0,
            }
            memory.program_counter += 2;
        }
        Instruction::SetBCD { vx } => {
            let hundreds = memory.registers[vx] / 100;
            let tens = (memory.registers[vx] - (hundreds * 100)) / 10;
            let ones = memory.registers[vx] - (hundreds * 100) - (tens * 10);

            memory.ram[memory.i as usize] = hundreds;
            memory.ram[memory.i as usize + 1] = tens;
            memory.ram[memory.i as usize + 2] = ones;

            memory.program_counter += 2;
        }
        Instruction::LoadRegisters { vx } => {
            for index in 0..(vx + 1) {
                memory.ram[memory.i as usize + index] = memory.registers[index];
            }
            memory.program_counter += 2;
        }
        Instruction::ReadRegisters { vx } => {
            for index in 0..(vx + 1) {
                memory.registers[index] = memory.ram[memory.i as usize + index];
            }
            memory.program_counter += 2;
        }
    }

    None
}

/// Returns the top 4 bits of a u8
fn get_upper_bits(byte: u8) -> u8 {
    (byte & 0b1111_0000) >> 4
}

/// Returns the lower 4 bits of a u8
fn get_lower_bits(byte: u8) -> u8 {
    byte & 0b0000_1111
}

fn get_as_bits(byte: u8) -> [u8; 8] {
    let mut bits = [0; 8];
    bits[0] = (byte & 0b1000_0000) >> 7;
    bits[1] = (byte & 0b0100_0000) >> 6;
    bits[2] = (byte & 0b0010_0000) >> 5;
    bits[3] = (byte & 0b0001_0000) >> 4;
    bits[4] = (byte & 0b0000_1000) >> 3;
    bits[5] = (byte & 0b0000_0100) >> 2;
    bits[6] = (byte & 0b0000_0010) >> 1;
    bits[7] = byte & 0b0000_0001;

    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_upper_bits() {
        let byte = 0xF5;
        assert_eq!(get_upper_bits(byte), 0x0F)
    }

    #[test]
    fn test_get_lower_bits() {
        let byte = 0xF5;
        assert_eq!(get_lower_bits(byte), 0x05)
    }

    #[test]
    fn test_get_as_bits() {
        let byte = 0b1010_1010;
        assert_eq!(get_as_bits(byte), [1, 0, 1, 0, 1, 0, 1, 0])
    }
}
