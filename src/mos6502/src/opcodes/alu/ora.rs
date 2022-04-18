use bus::Bus;
use crate::{Mos6502, Instruction, Flags::*};

/// ORA - Logical Inclusive OR
/// A,Z,N = A|M
///
/// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of
/// a byte of memory.
pub fn ora(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, additional_cycle) = cpu.address_mode_fetch(bus, &inst);
    cpu.a = cpu.a | fetched;
    cpu.write_flag_cond(Zero, cpu.a == 0);
    cpu.write_flag_cond(Negative, cpu.a & 0x80 == 0x80);
    cpu.pc += inst.bytes as u16;
    additional_cycle
}

#[cfg(test)]
mod tests {
    use crate::{Absolute, AbsoluteX, AbsoluteY, Immediate, IndirectX, IndirectY, ZeroPage, ZeroPageX};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME:&str = "ORA";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn immediate() {
        let opcode = OPTABLE[0x09];
        assert_eq!(opcode.mode, Immediate);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // Zero flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_0000;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 0b0000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_0000);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // Negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_0000;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 0b1000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b1000_0000);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page() {
        let opcode = OPTABLE[0x05];
        assert_eq!(opcode.mode, ZeroPage);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page_x() {
        let opcode = OPTABLE[0x15];
        assert_eq!(opcode.mode, ZeroPageX);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        cpu.x = 0x01;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x11, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0x0D];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1234, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_x() {
        let opcode = OPTABLE[0x1D];
        assert_eq!(opcode.mode, AbsoluteX);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        cpu.x = 0x01;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1235, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // with page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        cpu.x = 0xff;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1333, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_y() {
        let opcode = OPTABLE[0x19];
        assert_eq!(opcode.mode, AbsoluteY);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        cpu.y = 0x01;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1235, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // with page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        cpu.y = 0xff;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1333, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn indirect_x() {
        let opcode = OPTABLE[0x01];
        assert_eq!(opcode.mode, IndirectX);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn indirect_y() {
        let opcode = OPTABLE[0x11];
        assert_eq!(opcode.mode, IndirectY);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        cpu.y = 0x1;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234 + cpu.y as u16, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // with page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x0800;
        cpu.y = 0xff;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234 + cpu.y as u16, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0000_1110);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }
}