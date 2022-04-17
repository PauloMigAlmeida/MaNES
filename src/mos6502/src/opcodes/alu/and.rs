use bus::Bus;
use crate::Flags::{Negative, Zero};
use super::Mos6502;
use super::Instruction;

/// AND - Logical AND
/// A,Z,N = A&M
///
/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a
/// byte of memory.
pub fn and(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, additional_cycle) = cpu.address_mode_fetch(bus, &inst);
    cpu.a = cpu.a & fetched;
    cpu.set_flag_cond(Zero, cpu.a == 0);
    cpu.set_flag_cond(Negative, cpu.a & 0x80 == 0x80 );
    cpu.pc += inst.bytes as u16;
    additional_cycle
}

#[cfg(test)]
mod tests{

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

        // Indirect X mode, no flags
        let opcode = OPTABLE[0x21];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Indirect Y mode, no flags
        let opcode = OPTABLE[0x31];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x1;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234 + cpu.y as u16, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);


        // Indirect Y mode, no flags, page crossed
        let opcode = OPTABLE[0x31];
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0xff;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x34);
        bus.write_u8(0x34, 0x34);
        bus.write_u8(0x35, 0x12);
        bus.write_u8(0x1234 + cpu.y as u16, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }
}