use crate::instructions::Address;
use crate::instructions::Instruction;
use crate::memory::Memory;
use rand::Rng;
use std::thread;
use std::time;

pub fn parse(high_byte: u8, low_byte: u8) -> Instruction {
    match high_byte {
        0x00 => match low_byte {
            0xE0 => Instruction::Clear,
            0xEE => Instruction::Return,
            _ => Instruction::Invalid,
        },
        //1nnn
        _byte if get_upper_bits(high_byte) == 1 => Instruction::JumpTo(Address {
            high: get_lower_bits(high_byte),
            middle: get_upper_bits(low_byte),
            low: get_lower_bits(low_byte),
        }),
        //2nnn
        _byte if get_upper_bits(high_byte) == 2 => Instruction::Call(Address {
            high: get_lower_bits(high_byte),
            middle: get_upper_bits(low_byte),
            low: get_lower_bits(low_byte),
        }),
        //3xkk
        _byte if get_upper_bits(high_byte) == 3 => Instruction::SkipIfEqualByte {
            vx: get_lower_bits(high_byte) as usize,
            byte: low_byte,
        },
        //4xkk
        _byte if get_upper_bits(high_byte) == 4 => Instruction::SkipIfNotEqualByte {
            vx: get_lower_bits(high_byte) as usize,
            byte: low_byte,
        },
        //5xy0
        // TODO: also check lowest 4 bits are 0?
        _byte if get_upper_bits(high_byte) == 5 => Instruction::SkipIfEqualReg {
            vx: get_lower_bits(high_byte) as usize,
            vy: get_upper_bits(low_byte) as usize,
        },
        //6xkk
        _byte if get_upper_bits(high_byte) == 6 => Instruction::LoadByte {
            vx: get_lower_bits(high_byte) as usize,
            byte: low_byte,
        },
        //7xkk
        _byte if get_upper_bits(high_byte) == 7 => Instruction::AddByte {
            vx: get_lower_bits(high_byte) as usize,
            byte: low_byte,
        },
        //8xy_
        _byte if get_upper_bits(high_byte) == 8 => {
            match low_byte {
                //8xy0
                _lbyte if get_lower_bits(low_byte) == 0 => Instruction::LoadReg {
                    vx: get_lower_bits(high_byte) as usize,
                    vy: get_upper_bits(low_byte) as usize,
                },
                //8xy1
                _lbyte if get_lower_bits(low_byte) == 1 => Instruction::OR {
                    vx: get_lower_bits(high_byte) as usize,
                    vy: get_upper_bits(low_byte) as usize,
                },
                //8xy2
                _lbyte if get_lower_bits(low_byte) == 2 => Instruction::AND {
                    vx: get_lower_bits(high_byte) as usize,
                    vy: get_upper_bits(low_byte) as usize,
                },
                //8xy3
                _lbyte if get_lower_bits(low_byte) == 3 => Instruction::XOR {
                    vx: get_lower_bits(high_byte) as usize,
                    vy: get_upper_bits(low_byte) as usize,
                },
                //8xy4
                _lbyte if get_lower_bits(low_byte) == 4 => Instruction::AddReg {
                    vx: get_lower_bits(high_byte) as usize,
                    vy: get_upper_bits(low_byte) as usize,
                },
                //8xy5
                _lbyte if get_lower_bits(low_byte) == 5 => Instruction::Subtract {
                    vx: get_lower_bits(high_byte) as usize,
                    vy: get_upper_bits(low_byte) as usize,
                },
                //8xy6
                _lbyte if get_lower_bits(low_byte) == 6 => Instruction::ShiftRight {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //8xy7
                _lbyte if get_lower_bits(low_byte) == 7 => Instruction::SubtractReverse {
                    vx: get_lower_bits(high_byte) as usize,
                    vy: get_upper_bits(low_byte) as usize,
                },
                //8xyE
                _lbyte if get_lower_bits(low_byte) == 0xE => Instruction::ShiftLeft {
                    vx: get_lower_bits(high_byte) as usize,
                },
                _ => Instruction::Invalid,
            }
        }
        //9xy0
        // TODO: also check lowest 4 bits are 0?
        _byte if get_upper_bits(high_byte) == 9 => Instruction::SkipIfNotEqualReg {
            vx: get_lower_bits(high_byte) as usize,
            vy: get_upper_bits(low_byte) as usize,
        },
        //Annn
        _byte if get_upper_bits(high_byte) == 0xA => Instruction::LoadAddress(Address {
            high: get_lower_bits(high_byte),
            middle: get_upper_bits(low_byte),
            low: get_lower_bits(low_byte),
        }),
        //Bnnn
        _byte if get_upper_bits(high_byte) == 0xB => Instruction::JumpOffset(Address {
            high: get_lower_bits(high_byte),
            middle: get_upper_bits(low_byte),
            low: get_lower_bits(low_byte),
        }),
        //Cxkk
        _byte if get_upper_bits(high_byte) == 0xC => Instruction::Random {
            vx: get_lower_bits(high_byte) as usize,
            byte: low_byte,
        },
        //Dxyn
        _byte if get_upper_bits(high_byte) == 0xD => Instruction::Draw {
            vx: get_lower_bits(high_byte) as usize,
            vy: get_upper_bits(low_byte) as usize,
            nibble: get_lower_bits(low_byte),
        },
        //Ex__
        _byte if get_upper_bits(high_byte) == 0xE => {
            match low_byte {
                //Ex9E
                _lbyte if low_byte == 0x9E => Instruction::SkipIfKeyPressed {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //ExA1
                _lbyte if low_byte == 0xA1 => Instruction::SkipIfNotKeyPressed {
                    vx: get_lower_bits(high_byte) as usize,
                },
                _ => Instruction::Invalid,
            }
        }
        //Fx__
        _byte if get_upper_bits(high_byte) == 0xF => {
            match low_byte {
                //Fx07
                _lbyte if low_byte == 0x07 => Instruction::LoadDelay {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //Fx0A
                _lbyte if low_byte == 0x0A => Instruction::LoadKeyPressed {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //Fx15
                _lbyte if low_byte == 0x15 => Instruction::SetDelay {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //Fx18
                _lbyte if low_byte == 0x18 => Instruction::SetSound {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //Fx1E
                _lbyte if low_byte == 0x1E => Instruction::AddAddressOffset {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //Fx29
                _lbyte if low_byte == 0x29 => Instruction::LoadSprite {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //Fx33
                _lbyte if low_byte == 0x33 => Instruction::SetBCD {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //Fx55
                _lbyte if low_byte == 0x55 => Instruction::LoadRegisters {
                    vx: get_lower_bits(high_byte) as usize,
                },
                //Fx65
                _lbyte if low_byte == 0x65 => Instruction::ReadRegisters {
                    vx: get_lower_bits(high_byte) as usize,
                },
                _ => Instruction::Invalid,
            }
        }
        _ => Instruction::Invalid,
    }
}

pub fn execute(memory: &mut Memory, instruction: Instruction) {
    // Because we read in the instruction and arguments together,
    // we need to increment the program counter by 2 normally so we don't read in the middle of anything.
    match instruction {
        Instruction::Invalid => todo!(),
        Instruction::Clear => {
            memory.display = [[0; 64]; 32];
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
            memory.registers[vx] = memory.registers[vx] + memory.registers[vy];
            memory.program_counter += 2;
        }
        Instruction::OR { vx, vy } => {
            memory.registers[vx] = memory.registers[vx] | memory.registers[vy];
            memory.program_counter += 2;
        }
        Instruction::AND { vx, vy } => {
            memory.registers[vx] = memory.registers[vx] & memory.registers[vy];
            memory.program_counter += 2;
        }
        Instruction::XOR { vx, vy } => {
            memory.registers[vx] = memory.registers[vx] ^ memory.registers[vy];
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
                memory.registers[vx] = memory.registers[vx] - memory.registers[vy];
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
            memory.registers[vx] = memory.registers[vx] / 2;
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
            memory.registers[vx] = memory.registers[vx] << 1;
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

                for col in 0..8 {
                    let old_value = memory.display[(y + row) % 32][(x + col) % 64];
                    memory.display[(y + row) % 32][(x + col) % 64] ^= bits[col];

                    if old_value == 1 && memory.display[(y + row) % 32][(x + col) % 64] == 0 {
                        collided = true;
                    }
                }
            }

            if collided {
                memory.registers[0xF] = 1;
            } else {
                memory.registers[0xF] = 0;
            }

            for i in 0..32 {
                for j in 0..64 {
                    if memory.display[i][j] == 1 {
                        // print out unicode 'FULL BLOCK' U+2588
                        print!("{}", String::from_utf8(vec![0xE2, 0x96, 0x88]).unwrap());
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }

            thread::sleep(time::Duration::from_secs(1));

            memory.program_counter += 2;
        }
        Instruction::SkipIfKeyPressed { vx: _ } => todo!(),
        Instruction::SkipIfNotKeyPressed { vx: _ } => todo!(),
        Instruction::LoadDelay { vx } => {
            memory.registers[vx] = memory.delay;
            memory.program_counter += 2;
        }
        Instruction::LoadKeyPressed { vx: _ } => todo!(),
        Instruction::SetDelay { vx } => {
            memory.delay = memory.registers[vx];
            memory.program_counter += 2;
        }
        Instruction::SetSound { vx } => {
            memory.sound = memory.registers[vx];
            memory.program_counter += 2;
        }
        Instruction::AddAddressOffset { vx } => {
            memory.i = memory.i + memory.registers[vx] as u16;
            memory.program_counter += 2;
        }
        Instruction::LoadSprite { vx: _ } => todo!(),
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
