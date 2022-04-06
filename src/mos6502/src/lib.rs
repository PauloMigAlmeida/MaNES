mod opcodes;

use bus::Bus;
use opcodes::{Instruction, AddressingMode, parse_instruction};

#[derive(Debug)]
pub struct Mos6502 {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    flags: u8,
}

impl Mos6502 {
    pub fn new() -> Self {
        //TODO find out default values for the CPU
        Mos6502 {
            a: 0x0,
            x: 0x0,
            y: 0x0,
            pc: 0x0,
            sp: 0xFD,
            flags: 0x34,
        }
    }

    pub fn execute_instruction(&mut self, opcode: u8, bus: &Bus) {
        let inst = parse_instruction(opcode);
        (inst.function)(self, inst.mode, bus);
    }
}