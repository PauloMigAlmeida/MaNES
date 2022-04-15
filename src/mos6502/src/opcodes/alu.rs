use bus::Bus;
use crate::Flags::{Negative, Zero};
use super::{Mos6502, AddressingMode::*, Instruction};

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn ora(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn adc(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn and(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let mut additional_cycle = 0_u8;

    let fetched= match inst.mode {
        Immediate => bus.read_u8(cpu.pc + 1),
        ZeroPage => {
            let addr = bus.read_u8(cpu.pc + 1);
            bus.read_u8(addr as u16)
        },
        ZeroPageX => {
            // val = PEEK((arg + X) % 256) to simulate hardware bug in 6502
            let mut addr = bus.read_u8(cpu.pc + 1) as u16;
            addr = (addr + cpu.x as u16) % 256;
            bus.read_u8(addr)
        },
        Absolute => {
            let addr = bus.read_u16(cpu.pc + 1);
            bus.read_u8(addr)
        },
        AbsoluteX => {
            let orig_addr = bus.read_u16(cpu.pc + 1);
            let addr = orig_addr + cpu.x as u16;

            // page crossing costs 1 additional cycle.. Joao would be proud of me now <3
            if (orig_addr >> 8) != (addr >> 8) {
                additional_cycle = 1;
            }

            bus.read_u8(addr)
        },
        AbsoluteY => {
            let orig_addr = bus.read_u16(cpu.pc + 1);
            let addr = orig_addr + cpu.y as u16;

            // page crossing costs 1 additional cycle
            if (orig_addr >> 8) != (addr >> 8) {
                additional_cycle = 1;
            }

            bus.read_u8(addr)
        },
        IndirectX => {
            // val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256)
            let arg = bus.read_u8(cpu.pc + 1) as u16;
            let low = bus.read_u8((arg + cpu.x as u16) & 0xff) as u16;
            let high = bus.read_u8((arg + cpu.x as u16 + 1) & 0xff) as u16;
            bus.read_u8((high << 8) | low)
        }
        _ => panic!("invalid addressing mode... aborting"),
    };

    cpu.a = cpu.a & fetched;

    if cpu.a == 0 {
        cpu.set_flag(Zero);
    }

    if cpu.a & 0x80 == 0x80 {
        cpu.set_flag(Negative);
    }
    cpu.pc += inst.bytes as u16;
    additional_cycle
}

pub fn eor(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn sta(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn lda(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn cmp(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn sbc(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

#[cfg(test)]
mod test{

    use super::*;
    use crate::opcodes::{OPTABLE};

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn test_and() {
        // Immediate mode, no flags set
        let opcode = OPTABLE[0x29];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 0b0000_1000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1000);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // Immediate mode, zero flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 0b0000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_0000);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // Immediate mode, negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 0b1000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b1000_0000);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // ZeroPage mode, no flags
        let opcode = OPTABLE[0x25];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x0010, 0b0000_1111); // write to zero page
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // ZeroPage X mode, no flags
        let opcode = OPTABLE[0x35];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.x = 0x01;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x11, 0b0000_1111); // write to zero page
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // ZeroPage X mode (with hardware page wrap around 'bug'), no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.x = 0xFF;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0xF, 0b0001_1111); // write to zero page
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Absolute mode, no flags
        let opcode = OPTABLE[0x2D];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1234, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // Absolute X mode, no flags
        let opcode = OPTABLE[0x3D];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.x = 0x10;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1244, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // Absolute X mode, no flags, page crossed
        let opcode = OPTABLE[0x3D];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.x = 0xFF;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1333, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // Absolute Y mode, no flags
        let opcode = OPTABLE[0x39];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.y = 0x10;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1244, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // Absolute Y mode, no flags, page crossed
        let opcode = OPTABLE[0x39];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.y = 0xFF;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1333, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }
}