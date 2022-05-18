use super::*;

/// CPY - Compare Y Register
/// Z,C,N = Y-M
///
/// This instruction compares the contents of the Y register with another memory held value and
/// sets the zero and carry flags as appropriate.
pub fn cpy(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, additional_cycle) = cpu.address_mode_fetch(bus, &inst);
    let result = cpu.y.overflowing_sub(fetched).0;
    cpu.write_flag_cond(Carry, cpu.y >= fetched);
    cpu.write_flag_cond(Zero, cpu.y == fetched);
    cpu.write_flag_cond(Negative,  result & 0x80 == 0x80);
    cpu.pc += inst.bytes as u16;
    additional_cycle
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME:&str = "CPY";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn immediate() {
        let opcode = OPTABLE[0xC0];
        assert_eq!(opcode.mode, Immediate);
        assert_eq!(opcode.name, OPCODE_NAME);

        // Carry flag
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x11;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x11);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // clear flags -> Carry flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b1000_0010;
        cpu.y = 0x11;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x11);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Carry + Zero flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x10;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.flags, 0b0000_0011);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page() {
        let opcode = OPTABLE[0xC4];
        assert_eq!(opcode.mode, ZeroPage);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x11;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x11);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0xCC];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x11;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1234, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x11);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }
}