use super::*;

/// INC - Decrement Memory
/// M,Z,N = M-1
///
/// Subtracts one from the value held at a specified memory location setting the zero and negative
/// flags as appropriate.
pub fn dec(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, _) = cpu.address_mode_fetch(bus, &inst);
    let result = fetched.overflowing_sub(1).0;

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

    cpu.write_flag_cond(Zero, result == 0);
    cpu.write_flag_cond(Negative, result & 0x80 == 0x80);
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME: &str = "DEC";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn zero_page() {
        let opcode = OPTABLE[0xC6];
        assert_eq!(opcode.mode, ZeroPage);
        assert_eq!(opcode.name, OPCODE_NAME);

        // No flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x0;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x10, 0b0000_0010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(bus.cpu_read_u8(0x10), 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // test zero, negative flags clear
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0010;
        cpu.y = 0x0;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x10, 0b0000_0010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(bus.cpu_read_u8(0x10), 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Zero flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x0;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x10, 0b0000_0001);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(bus.cpu_read_u8(0x10), 0x0);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // Negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x0;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x10, 0b0000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(bus.cpu_read_u8(0x10), 0b1111_1111); // -1 two's complement
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // negative flag set, decrement negative numbers
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x0;
        cpu.pc = 0x0800;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x10, 0b1111_0110); // -10 two's complement
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(bus.cpu_read_u8(0x10), 0b1111_0101); // -11 two's complement
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page_x() {
        let opcode = OPTABLE[0xD6];
        assert_eq!(opcode.mode, ZeroPageX);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x0;
        cpu.pc = 0x0800;
        cpu.x = 0x01;
        bus.cpu_write_u8(cpu.pc + 1, 0x10);
        bus.cpu_write_u8(0x11, 0x2);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(bus.cpu_read_u8(0x11), 0x1);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0xCE];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x0;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1234, 0x2);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(bus.cpu_read_u8(0x1234), 0x1);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_x() {
        let opcode = OPTABLE[0xDE];
        assert_eq!(opcode.mode, AbsoluteX);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page cross
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x0;
        cpu.x = 0x1;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1235, 0x2);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(bus.cpu_read_u8(0x1235), 0x1);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // page cross (NO additional cycle)
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0x0;
        cpu.x = 0xff;
        cpu.pc = 0x0800;
        bus.cpu_write_u16(cpu.pc + 1, 0x1234);
        bus.cpu_write_u8(0x1333, 0x2);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(bus.cpu_read_u8(0x1333), 0x1);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }
}