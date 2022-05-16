pub const INTERPRETER_SIZE: usize = 0x200;

#[derive(Debug)]
pub struct Memory {
    pub delay: u8,
    pub sound: u8,
    pub i: u16,
    pub program_counter: u16,
    pub stack_pointer: usize,
    pub registers: [u8; 16],
    pub stack: [u16; 16],
    pub display: [[u8; 64]; 32],
    pub ram: [u8; 0xFFF],
}
