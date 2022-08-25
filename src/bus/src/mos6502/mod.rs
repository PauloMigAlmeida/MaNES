mod opcodes;

pub use crate::Bus;
use opcodes::{Flags, Flags::*, parse_instruction};
pub use crate::mos6502::opcodes::{AddressingMode, Instruction};
pub use crate::mos6502::opcodes::OPTABLE;
pub use crate::traits::MainBusConnection;

const STACK_PAGE:u16 = 0x0100;

#[derive(Debug, Clone)]
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

    pub fn reset(&mut self, bus: &Bus) {
        // Get address to set program counter to
        self.pc = bus.cpu_read_u16(0xFFFC, true);

        // reset regs
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;

        self.flags = 0x0;
        self.set_flag(Unused);

        // Reset takes time
        self.cycles = 8;
    }

    //TODO implement irq

    //TODO implement tests for nmi
    pub fn nmi(&mut self, bus: &mut Bus) {
        self.stack_push(((self.pc >> 8) & 0xff) as u8, bus);
        self.stack_push((self.pc & 0xff) as u8, bus);

        self.clear_flag(Break);
        self.set_flag(Unused);
        self.set_flag(DisableInterrupt);
        self.stack_push(self.flags, bus);

        self.pc = bus.cpu_read_u16(0xFFFA, true);
        self.cycles = 8;

    }

    //TODO implement clock

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
        bus.cpu_write_u8(addr, value);
        self.sp -= 1;
    }

    pub fn stack_pull(&mut self, bus: &Bus) -> u8 {
        if self.sp == 0xFF {
            panic!("Can't pull more data from the stack");
        }

        self.sp += 1;
        let addr: u16 = STACK_PAGE | self.sp  as u16;
        bus.cpu_read_u8(addr, false)
    }

    // Notes to myself
    // -> TODO: I'm not yet 100% confident that I got the inner workings of Indirect X && Y
    pub fn address_mode_fetch(&self, bus: &Bus, inst: &Instruction) -> (u8, u8) {
        let mut additional_cycle= 0;

        let fetched= match inst.mode {
            AddressingMode::Immediate | AddressingMode::Relative => bus.cpu_read_u8(self.pc + 1, false),
            AddressingMode::Accumulator => self.a,
            AddressingMode::ZeroPage => {
                let addr = bus.cpu_read_u8(self.pc + 1, false);
                bus.cpu_read_u8(addr as u16, false)
            },
            AddressingMode::ZeroPageX => {
                // val = PEEK((arg + X) % 256) to simulate hardware bug in 6502
                let mut addr = bus.cpu_read_u8(self.pc + 1, false) as u16;
                addr = (addr + self.x as u16) % 256;
                bus.cpu_read_u8(addr, false)
            },
            AddressingMode::ZeroPageY => {
                // val = PEEK((arg + Y) % 256) to simulate hardware bug in 6502
                let mut addr = bus.cpu_read_u8(self.pc + 1, false) as u16;
                addr = (addr + self.y as u16) % 256;
                bus.cpu_read_u8(addr, false)
            },
            AddressingMode::Absolute => {
                let addr = bus.cpu_read_u16(self.pc + 1, false);
                bus.cpu_read_u8(addr, false)
            },
            AddressingMode::AbsoluteX => {
                let orig_addr = bus.cpu_read_u16(self.pc + 1, false);
                let addr = orig_addr + self.x as u16;

                // page crossing costs 1 additional cycle.. Joao would be proud of me now <3
                if (orig_addr >> 8) != (addr >> 8) {
                    additional_cycle = 1;
                }

                bus.cpu_read_u8(addr, false)
            },
            AddressingMode::AbsoluteY => {
                let orig_addr = bus.cpu_read_u16(self.pc + 1, false);
                let addr = orig_addr + self.y as u16;

                // page crossing costs 1 additional cycle
                if (orig_addr >> 8) != (addr >> 8) {
                    additional_cycle = 1;
                }

                bus.cpu_read_u8(addr, false)
            },
            AddressingMode::IndirectX => {
                // val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256)
                let arg = bus.cpu_read_u8(self.pc + 1, false) as u16;
                let low = bus.cpu_read_u8((arg + self.x as u16) & 0xff, false) as u16;
                let high = bus.cpu_read_u8((arg + self.x as u16 + 1) & 0xff, false) as u16;
                bus.cpu_read_u8((high << 8) | low, false)
            },
            AddressingMode::IndirectY => {
                // val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y)
                let arg = bus.cpu_read_u8(self.pc + 1, false) as u16;
                let low = bus.cpu_read_u8(arg  & 0xff, false) as u16;
                let high = bus.cpu_read_u8((arg + 1) & 0xff, false) as u16;

                let orig_addr = (high << 8) | low;
                let addr = orig_addr + self.y as u16;

                // page crossing costs 1 additional cycle
                if (orig_addr >> 8) != (addr >> 8) {
                    additional_cycle = 1;
                }

                bus.cpu_read_u8(addr, false)
            },
            _ => panic!("invalid addressing mode... aborting"),
        };
        (fetched, additional_cycle)
    }

}

#[cfg(test)]
mod test;