use super::*;

/// BIT - Bit Test
/// A & M, N = M7, V = M6
///
/// This instructions is used to test if one or more bits are set in a 
/// target memory location. The mask pattern in A is ANDed with the value
/// in memory to set or clear the zero flag, but the result is not kept.
/// Bits 7 and 6 of the value from memory are copied into the N and V flags.
pub fn bit(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, _) = cpu.address_mode_fetch(bus, &inst);
    cpu.write_flag_cond(Zero, cpu.a & fetched == 0);
    cpu.write_flag_cond(Overflow, fetched & 0x40 == 0x40);
    cpu.write_flag_cond(Negative, fetched & 0x80 == 0x80);
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME: &str = "BIT";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn zero_page() {
        let opcode = OPTABLE[0x24];
        assert_eq!(opcode.mode, ZeroPage);
        assert_eq!(opcode.name, OPCODE_NAME);

        // No flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x1;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0x1);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.a, 0x1);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Zero flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0x1);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Overflow and Zero flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0x40);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0100_0010);
        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Negative and Zero flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0x80);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b1000_0010);
        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x1;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0x81);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.a, 0x1);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Overflow flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x1;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0x41);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0100_0000);
        assert_eq!(cpu.a, 0x1);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0x2C];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        // No flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x1;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1234, 0x1);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.a, 0x1);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

}