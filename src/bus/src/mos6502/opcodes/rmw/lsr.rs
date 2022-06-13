use super::*;

/// LSR - Logical Shift Right
/// A,C,Z,N = A/2 or M,C,Z,N = M/2
///
/// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is
/// shifted into the carry flag. Bit 7 is set to zero.
pub fn lsr(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, _) = cpu.address_mode_fetch(bus, &inst);
    let result = fetched >> 1;

    if inst.mode == Accumulator{
        cpu.a = result;
    } else {
        let addr = match inst.mode {
            ZeroPage => {
                bus.cpu_read_u8(cpu.pc + 1) as u16
            },
            ZeroPageX => {
                let addr = bus.cpu_read_u8(cpu.pc + 1) as u16;
                (addr + cpu.x as u16) % 256
            },
            Absolute => {
                bus.cpu_read_u16(cpu.pc + 1)
            },
            AbsoluteX => {
                let orig_addr = bus.cpu_read_u16(cpu.pc + 1);
                orig_addr + cpu.x as u16
            },
            _ => unreachable!("invalid addressing mode... aborting"),
        };
        bus.cpu_write_u8(addr, result);
    }

    cpu.write_flag_cond(Carry, fetched & 0x1 == 0x1);
    cpu.write_flag_cond(Zero, result == 0);
    // TODO: Verify that after running cpu integration tests...
    // in the docs it says that this flag can be set but I'm struggling to believe that this is even
    // achievable... then again, 0xff >> 1 = 0x7f which doesn't have bit 7 set... and that's the
    // largest number it can hold ¯\_(ツ)_/¯
    cpu.write_flag_cond(Negative, result & 0x80 == 0x80);
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME: &str = "LSR";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn accumulator() {
        let opcode = OPTABLE[0x4A];
        assert_eq!(opcode.mode, Accumulator);
        assert_eq!(opcode.name, OPCODE_NAME);

        // No flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_0010;
        cpu.pc = 0x0800;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);

        // Clear flags, No flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0010;
        cpu.a = 0b0000_0010;
        cpu.pc = 0x0800;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);

        // Zero flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_000;
        cpu.a = 0b0000_0000;
        cpu.pc = 0x0800;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_0000);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);

        // Carry flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_000;
        cpu.a = 0b0000_1001;
        cpu.pc = 0x0800;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_0100);
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page() {
        let opcode = OPTABLE[0x46];
        assert_eq!(opcode.mode, ZeroPage);
        assert_eq!(opcode.name, OPCODE_NAME);

        // No flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x10, 0b0000_0010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0);
        assert_eq!(bus.cpu_read_u8(0x10), 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page_x() {
        let opcode = OPTABLE[0x56];
        assert_eq!(opcode.mode, ZeroPageX);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.pc = 0x0800;
        cpu.x = 0x01;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x11, 0b0000_0010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0x1);
        assert_eq!(bus.cpu_read_u8(0x11), 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0x4E];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0x0;
        cpu.a = 0x0;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1234, 0b0000_0010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(bus.cpu_read_u8(0x1234), 0b0000_0001);
        assert_eq!(cpu.x, 0x0);
        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_x() {
        let opcode = OPTABLE[0x5E];
        assert_eq!(opcode.mode, AbsoluteX);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page cross
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x0;
        cpu.x = 0x1;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1235, 0b0000_0010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(bus.cpu_read_u8(0x1235), 0b0000_0001);
        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.x, 0x1);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // page cross (NO additional cycle)
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x0;
        cpu.x = 0xff;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1333, 0b0000_0010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.x, 0xff);
        assert_eq!(bus.cpu_read_u8(0x1333), 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }
}