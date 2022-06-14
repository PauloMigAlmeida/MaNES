use super::*;

/// PLA - Pull Accumulator
/// Pulls an 8 bit value from the stack and into the accumulator.
/// The zero and negative flags are set as appropriate.
pub fn pla(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let value = cpu.stack_pull(bus);
    if value == 0 {
        cpu.set_flag(Zero);
    }
    if (value & (1 << 7)) != 0 {
        cpu.set_flag(Negative);
    }
    cpu.a = value;
    cpu.pc += 1;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME:&str = "PLA";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0x68];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags set
        let (mut cpu, mut bus) = init();
        bus.cpu_write_u8(0x1ff, 0xEE);
        cpu.sp = 0xfe;
        cpu.pc = 0x0800;
        cpu.flags = 0b1100_1111;
        cpu.a = 0x10;
        cpu.x = 0;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b1100_1111);
        assert_eq!(cpu.a, 0xEE);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);

        // Test zero flag
        let (mut cpu, mut bus) = init();
        bus.cpu_write_u8(0x1ff, 0x0);
        cpu.sp = 0xfe;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x10;
        cpu.x = 0;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);

        // Test negative flag
        let (mut cpu, mut bus) = init();
        bus.cpu_write_u8(0x1ff, 0x81);
        cpu.sp = 0xfe;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x10;
        cpu.x = 0;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.a, 0x81);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);
    }
}