mod opcodes;

use bus::Bus;
use opcodes::{Instruction, AddressingMode, parse_instruction};

#[derive(Debug)]
pub struct Mos6502 {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    pc: u16,
    sp: u8,
    st_flags: u8,
}

impl Mos6502 {
    pub fn new() -> Self {
        //TODO find out default values for the CPU
        Mos6502 {
            reg_a: 0x0,
            reg_x: 0x0,
            reg_y: 0x0,
            pc: 0x0,
            sp: 0x0,
            st_flags: 0x0,
        }
    }

    pub fn execute_instruction(&mut self, opcode: u8, bus: &Bus) {
        let inst = parse_instruction(opcode);
        (inst.function)(self, inst.addressing_mode, bus);
    }
}