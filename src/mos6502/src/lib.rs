mod opcodes;

use bus::Bus;
use opcodes::{Instruction, AddressingMode, parse_instruction, Flags};

const STACK_PAGE:u16 = 0x0100;

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
            /* offset for the 0x0100 page */
            flags: 0x34,
        }
    }

    pub fn execute_instruction(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        let inst = parse_instruction(opcode);
        let r = (inst.function)(self, inst, bus);

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

    pub fn stack_push(&mut self, value: u8, bus: &mut Bus) {
        if self.sp == 0 {
            panic!("Can't push more data into the stack");
        }

        let addr: u16 = STACK_PAGE | self.sp as u16;
        bus.write_address(addr, value);
        self.sp -= 1;
    }

    pub fn stack_pull(&mut self, bus: &Bus) -> u8 {
        if self.sp == 0xFF {
            panic!("Can't pull more data from the stack");
        }

        self.sp += 1;
        let addr: u16 = STACK_PAGE | self.sp  as u16;
        let value = bus.read_address(addr);
        value
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

    #[test]
    fn test_stack_push() {
        let mut cpu = Mos6502::new();
        let mut bus = Bus::new();

        cpu.sp = 0xff;
        cpu.stack_push(0x10, &mut bus);
        assert_eq!(cpu.sp, 0xfe);
        cpu.stack_push(0x11, &mut bus);
        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(bus.read_address(STACK_PAGE | 0xff), 0x10);
        assert_eq!(bus.read_address(STACK_PAGE | 0xfe), 0x11);
    }

    #[test]
    #[should_panic]
    fn test_stack_push_overflow() {
        let mut cpu = Mos6502::new();
        let mut bus = Bus::new();

        cpu.sp = 0x00;
        cpu.stack_push(0x10, &mut bus);
    }

    #[test]
    fn test_stack_pull() {
        let mut cpu = Mos6502::new();
        let mut bus = Bus::new();

        cpu.sp = 0xff;
        cpu.stack_push(0x10, &mut bus);
        cpu.stack_push(0x11, &mut bus);
        assert_eq!(cpu.stack_pull(&bus), 0x11);
        assert_eq!(cpu.sp, 0xfe);
        assert_eq!(cpu.stack_pull(&bus), 0x10);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    #[should_panic]
    fn test_stack_underflow() {
        let mut cpu = Mos6502::new();
        let bus = Bus::new();

        cpu.sp = 0xff;
        cpu.stack_pull(&bus);
    }
}