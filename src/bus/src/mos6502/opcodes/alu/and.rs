use super::*;

/// AND - Logical AND
/// A,Z,N = A&M
///
/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a
/// byte of memory.
pub fn and(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, additional_cycle) = cpu.address_mode_fetch(bus, &inst);
    cpu.a &= fetched;
    cpu.write_flag_cond(Zero, cpu.a == 0);
    cpu.write_flag_cond(Negative, cpu.a & 0x80 == 0x80 );
    cpu.pc += inst.bytes as u16;
    additional_cycle
}

#[cfg(test)]
mod tests{
    use super::*;

    const OPCODE_NAME:&str = "AND";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn immediate() {
        let opcode = OPTABLE[0x29];
        assert_eq!(opcode.mode, Immediate);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x10;
        bus.cpu_write_u8(cpu.pc + 1, 0b0000_1000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1000);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // test zero flag clear
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0010;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x10;
        bus.cpu_write_u8(cpu.pc + 1, 0b0000_1000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1000);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // zero flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x10;
        bus.cpu_write_u8(cpu.pc + 1, 0b0000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_0000);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x10;
        bus.cpu_write_u8(cpu.pc + 1, 0b1000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b1000_0000);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page() {
        let opcode = OPTABLE[0x25];
        assert_eq!(opcode.mode, ZeroPage);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x0010, 0b0000_1111); // write to zero page
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page_x() {
        let opcode = OPTABLE[0x35];
        assert_eq!(opcode.mode, ZeroPageX);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.x = 0x01;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x11, 0b0000_1111); // write to zero page
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // page wrap around, no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.x = 0xFF;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0xF, 0b0001_1111); // write to zero page
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0x2D];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1234, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_x() {
        let opcode = OPTABLE[0x3D];
        assert_eq!(opcode.mode, AbsoluteX);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.x = 0x10;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1244, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // no flags, page crossed
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.x = 0xFF;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1333, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_y() {
        let opcode = OPTABLE[0x39];
        assert_eq!(opcode.mode, AbsoluteY);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.y = 0x10;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1244, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // no flags, page crossed
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.y = 0xFF;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1333, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn indirect_x() {
        let opcode = OPTABLE[0x21];
        assert_eq!(opcode.mode, IndirectX);
        assert_eq!(opcode.name, OPCODE_NAME);

        // // no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x34);
        bus.cpu_write_u8(0x34, 0x34);
        bus.cpu_write_u8(0x35, 0x12);
        bus.cpu_write_u8(0x1234, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn indirect_y() {
        let opcode = OPTABLE[0x31];
        assert_eq!(opcode.mode, IndirectY);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x1;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x34);
        bus.cpu_write_u8(0x34, 0x34);
        bus.cpu_write_u8(0x35, 0x12);
        bus.cpu_write_u8(0x1234 + cpu.y as u16, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);


        // no flags, page crossed
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0xff;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x34);
        bus.cpu_write_u8(0x34, 0x34);
        bus.cpu_write_u8(0x35, 0x12);
        bus.cpu_write_u8(0x1234 + cpu.y as u16, 0b0001_1111);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.a, 0b0001_1111);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }
}