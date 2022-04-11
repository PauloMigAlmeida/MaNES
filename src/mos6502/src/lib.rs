mod opcodes;

use bus::Bus;
use opcodes::{Instruction, AddressingMode, parse_instruction, Flags};

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

    pub fn execute_instruction(&mut self, opcode: u8, bus: &Bus) -> u8 {
        let inst = parse_instruction(opcode);
        let r = (inst.function)(self, inst.mode, bus);
        self.pc += inst.bytes as u16;
        inst.cycles as u8 + r
    }

    pub fn set_flag(&mut self, flag: Flags) {
        let value = 1 << (flag as u8);
        self.flags |= value;
    }

    pub fn clear_flag(&mut self, flag: Flags) {
        let value = !(1 << (flag as u8));
        self.flags &= value;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_flag() {
        let mut cpu = Mos6502::new();
        cpu.flags = 0;

        cpu.set_flag(Flags::Carry);
        assert_eq!(cpu.flags, 0b0000_0001);
        cpu.set_flag(Flags::Zero);
        assert_eq!(cpu.flags, 0b0000_0011);
        cpu.set_flag(Flags::Interrupt);
        assert_eq!(cpu.flags, 0b0000_0111);
        cpu.set_flag(Flags::Decimal);
        assert_eq!(cpu.flags, 0b0000_1111);
        cpu.set_flag(Flags::Overflow);
        assert_eq!(cpu.flags, 0b0100_1111);
        cpu.set_flag(Flags::Negative);
        assert_eq!(cpu.flags, 0b1100_1111);
    }

    #[test]
    fn test_clear_flag() {
        let mut cpu = Mos6502::new();
        cpu.flags = 0b1100_1111;

        cpu.clear_flag(Flags::Carry);
        assert_eq!(cpu.flags, 0b1100_1110);
        cpu.clear_flag(Flags::Zero);
        assert_eq!(cpu.flags, 0b1100_1100);
        cpu.clear_flag(Flags::Interrupt);
        assert_eq!(cpu.flags, 0b1100_1000);
        cpu.clear_flag(Flags::Decimal);
        assert_eq!(cpu.flags, 0b1100_0000);
        cpu.clear_flag(Flags::Overflow);
        assert_eq!(cpu.flags, 0b1000_0000);
        cpu.clear_flag(Flags::Negative);
        assert_eq!(cpu.flags, 0b0000_0000);
    }
}