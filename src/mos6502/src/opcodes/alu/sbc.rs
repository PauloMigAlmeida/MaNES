use bus::Bus;
use crate::{Mos6502, Instruction, Flags::*};
/// SBC - Subtract with Carry
/// A,Z,C,N = A-M-(1-C)
///
/// This instruction subtracts the contents of a memory location to the accumulator together with
/// the not of the carry bit. If overflow occurs the carry bit is clear, this enables multiple byte
/// subtraction to be performed.
pub fn sbc(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, additional_cycle) = cpu.address_mode_fetch(bus, &inst);

    let (data1, is_carry1) = cpu.a.overflowing_sub(fetched);
    let (result, is_carry2) = data1.overflowing_sub(if cpu.is_flag_set(Carry) { 0 } else { 1 } );

    cpu.set_flag_cond(Carry, !(is_carry1 || is_carry2));
    cpu.set_flag_cond(Zero, result == 0);
    cpu.set_flag_cond(Negative, (result & 0x80) == 0x80);
    cpu.set_flag_cond(Overflow, (((cpu.a ^ fetched) & 0x80) == 0x80) && (((cpu.a ^ result) & 0x80) == 0x80));

    cpu.a = result;
    cpu.pc += inst.bytes as u16;
    additional_cycle
}

#[cfg(test)]
mod tests {
    use crate::{Absolute, AbsoluteX, AbsoluteY, Immediate, IndirectX, IndirectY, ZeroPage, ZeroPageX};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME:&str = "SBC";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn immediate() {
        let opcode = OPTABLE[0xE9];
        assert_eq!(opcode.mode, Immediate);
        assert_eq!(opcode.name, OPCODE_NAME);

        // Carry flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // No flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 12;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // Carry and Zero flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 0;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 0);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_0000);
        assert_eq!(cpu.flags, 0b0000_0011);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // Negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 10;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 10); // -10 two's complement
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 255);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // Overflow and Negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_0011;
        cpu.pc = 0x10;
        bus.write_u8(cpu.pc + 1, 0b1000_0010); // -126 two's complement
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 128);
        assert_eq!(cpu.flags, 0b1100_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page() {
        let opcode = OPTABLE[0xE5];
        assert_eq!(opcode.mode, ZeroPage);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page_x() {
        let opcode = OPTABLE[0xF5];
        assert_eq!(opcode.mode, ZeroPageX);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        cpu.x = 0x01;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x11, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0xED];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1234, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_x() {
        let opcode = OPTABLE[0xFD];
        assert_eq!(opcode.mode, AbsoluteX);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        cpu.x = 0x01;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1235, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // with page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        cpu.x = 0xff;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1333, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_y() {
        let opcode = OPTABLE[0xF9];
        assert_eq!(opcode.mode, AbsoluteY);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        cpu.y = 0x01;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1235, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // with page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        cpu.y = 0xff;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1333, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn indirect_x() {
        let opcode = OPTABLE[0xE1];
        assert_eq!(opcode.mode, IndirectX);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

    }

    #[test]
    fn indirect_y() {
        let opcode = OPTABLE[0xF1];
        assert_eq!(opcode.mode, IndirectY);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        cpu.y = 0x1;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234 + cpu.y as u16, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // with page crossing
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0001;
        cpu.a = 12;
        cpu.pc = 0x0800;
        cpu.y = 0xff;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234 + cpu.y as u16, 10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }
}