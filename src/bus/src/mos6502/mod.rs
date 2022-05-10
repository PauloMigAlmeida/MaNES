mod opcodes;

pub use crate::Bus;
use opcodes::{parse_instruction, Flags};
pub use crate::mos6502::opcodes::{AddressingMode, Instruction};
pub use crate::mos6502::opcodes::OPTABLE;
const STACK_PAGE:u16 = 0x0100;

#[derive(Debug)]
pub struct Mos6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub flags: u8,
    pub cycles: u8,
}

impl Mos6502 {
    pub fn new() -> Self {
        //TODO find out default values for the CPU
        Mos6502 {
            a: 0x0,
            x: 0x0,
            y: 0x0,
            pc: 0x0,
            /* offset for the 0x0100 page */
            sp: 0xFD,
            flags: 0x34,
            /*  counts how many cycles the instruction has remaining */
            cycles: 0,
        }
    }

    pub fn reset(&mut self, bus: & Bus) {
        // Get address to set program counter to
        self.pc = bus.read_u16(0xFFFC);

        // reset regs
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;

        self.flags = 0x0;
        self.set_flag(Flags::Unused);

        // Reset takes time
        self.cycles = 8;
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

    pub fn write_flag_cond(&mut self, flag: Flags, cond: bool) {
        if cond {
            self.set_flag(flag);
        }else {
            self.clear_flag(flag);
        }
    }

    pub fn is_flag_set(&self, flag: Flags) -> bool {
        let flag_value = flag as u8;
        self.flags & (1 << flag_value) == (1 << flag_value)
    }

    pub fn stack_push(&mut self, value: u8, bus: &mut Bus) {
        if self.sp == 0 {
            panic!("Can't push more data into the stack");
        }

        let addr: u16 = STACK_PAGE | self.sp as u16;
        bus.write_u8(addr, value);
        self.sp -= 1;
    }

    pub fn stack_pull(&mut self, bus: &Bus) -> u8 {
        if self.sp == 0xFF {
            panic!("Can't pull more data from the stack");
        }

        self.sp += 1;
        let addr: u16 = STACK_PAGE | self.sp  as u16;
        let value = bus.read_u8(addr);
        value
    }

    // Notes to myself
    // -> TODO: I'm not yet 100% confident that I got the inner workings of Indirect X && Y
    pub fn address_mode_fetch(&self, bus: &Bus, inst: &Instruction) -> (u8, u8) {
        let mut additional_cycle= 0;

        let fetched= match inst.mode {
            AddressingMode::Immediate | AddressingMode::Relative => bus.read_u8(self.pc + 1),
            AddressingMode::Accumulator => self.a,
            AddressingMode::ZeroPage => {
                let addr = bus.read_u8(self.pc + 1);
                bus.read_u8(addr as u16)
            },
            AddressingMode::ZeroPageX => {
                // val = PEEK((arg + X) % 256) to simulate hardware bug in 6502
                let mut addr = bus.read_u8(self.pc + 1) as u16;
                addr = (addr + self.x as u16) % 256;
                bus.read_u8(addr)
            },
            AddressingMode::ZeroPageY => {
                // val = PEEK((arg + Y) % 256) to simulate hardware bug in 6502
                let mut addr = bus.read_u8(self.pc + 1) as u16;
                addr = (addr + self.y as u16) % 256;
                bus.read_u8(addr)
            },
            AddressingMode::Absolute => {
                let addr = bus.read_u16(self.pc + 1);
                bus.read_u8(addr)
            },
            AddressingMode::AbsoluteX => {
                let orig_addr = bus.read_u16(self.pc + 1);
                let addr = orig_addr + self.x as u16;

                // page crossing costs 1 additional cycle.. Joao would be proud of me now <3
                if (orig_addr >> 8) != (addr >> 8) {
                    additional_cycle = 1;
                }

                bus.read_u8(addr)
            },
            AddressingMode::AbsoluteY => {
                let orig_addr = bus.read_u16(self.pc + 1);
                let addr = orig_addr + self.y as u16;

                // page crossing costs 1 additional cycle
                if (orig_addr >> 8) != (addr >> 8) {
                    additional_cycle = 1;
                }

                bus.read_u8(addr)
            },
            AddressingMode::IndirectX => {
                // val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256)
                let arg = bus.read_u8(self.pc + 1) as u16;
                let low = bus.read_u8((arg + self.x as u16) & 0xff) as u16;
                let high = bus.read_u8((arg + self.x as u16 + 1) & 0xff) as u16;
                bus.read_u8((high << 8) | low)
            },
            AddressingMode::IndirectY => {
                // val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y)
                let arg = bus.read_u8(self.pc + 1) as u16;
                let low = bus.read_u8(arg  & 0xff) as u16;
                let high = bus.read_u8((arg + 1) & 0xff) as u16;

                let orig_addr = (high << 8) | low;
                let addr = orig_addr + self.y as u16;

                // page crossing costs 1 additional cycle
                if (orig_addr >> 8) != (addr >> 8) {
                    additional_cycle = 1;
                }

                bus.read_u8(addr)
            },
            _ => panic!("invalid addressing mode... aborting"),
        };
        (fetched, additional_cycle)
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
        cpu.set_flag(Flags::DisableInterrupt);
        assert_eq!(cpu.flags, 0b0000_0111);
        cpu.set_flag(Flags::Decimal);
        assert_eq!(cpu.flags, 0b0000_1111);
        cpu.set_flag(Flags::Break);
        assert_eq!(cpu.flags, 0b0001_1111);
        cpu.set_flag(Flags::Overflow);
        assert_eq!(cpu.flags, 0b0101_1111);
        cpu.set_flag(Flags::Negative);
        assert_eq!(cpu.flags, 0b1101_1111);
    }

    #[test]
    fn test_clear_flag() {
        let mut cpu = Mos6502::new();
        cpu.flags = 0b1101_1111;

        cpu.clear_flag(Flags::Carry);
        assert_eq!(cpu.flags, 0b1101_1110);
        cpu.clear_flag(Flags::Zero);
        assert_eq!(cpu.flags, 0b1101_1100);
        cpu.clear_flag(Flags::DisableInterrupt);
        assert_eq!(cpu.flags, 0b1101_1000);
        cpu.clear_flag(Flags::Decimal);
        assert_eq!(cpu.flags, 0b1101_0000);
        cpu.clear_flag(Flags::Break);
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
        assert_eq!(bus.read_u8(STACK_PAGE | 0xff), 0x10);
        assert_eq!(bus.read_u8(STACK_PAGE | 0xfe), 0x11);
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

    // TODO implement vectors otherise this test will fail
    //
    // #[test]
    // fn test_cpu_reset() {
    //     let mut cpu = Mos6502::new();
    //     let mut bus = Bus::new();
    //
    //     bus.write_u16(0xFFFC, 0x1234);
    //     cpu.a = 0x1;
    //     cpu.x = 0x1;
    //     cpu.y = 0x1;
    //     cpu.sp = 0xC0;
    //     cpu.flags = 0xFF;
    //
    //     cpu.reset(&bus);
    //     assert_eq!(cpu.a, 0);
    //     assert_eq!(cpu.x, 0);
    //     assert_eq!(cpu.y, 0);
    //     assert_eq!(cpu.sp, 0xFD);
    //     assert_eq!(cpu.flags, 0b0010_0000);
    //     assert_eq!(cpu.cycles, 8);
    // }
}