use bus::Bus;
use crate::{Mos6502, Instruction, Flags::*};

/// LDX - Load X Register
/// Y,Z,N = M
///
/// Loads a byte of memory into the X register setting the zero and negative 
/// flags as appropriate.
pub fn ldx(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, additional_cycle) = cpu.address_mode_fetch(bus, &inst);
    cpu.x = fetched;
    cpu.write_flag_cond(Zero, cpu.x == 0);
    cpu.write_flag_cond(Negative, cpu.x & 0x80 == 0x80);
    cpu.pc += inst.bytes as u16;
    additional_cycle
}

#[cfg(test)]
mod tests {
    use crate::{Immediate, Absolute, AbsoluteY, ZeroPage, ZeroPageY};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME:&str = "LDX";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn immediate() {
        let opcode = OPTABLE[0xA2];
        assert_eq!(opcode.mode, Immediate);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags 
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0x10);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // clear flags 
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b1000_0010;
        cpu.x = 0;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0x10);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // zero flag set 
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0x10;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // negative flag set 
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0b1000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0b1000_0000);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page() {
        let opcode = OPTABLE[0xA6];
        assert_eq!(opcode.mode, ZeroPage);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0b0000_1100;
        cpu.pc = 0x0800;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x10, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0b0000_1010);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn zero_page_y() {
        let opcode = OPTABLE[0xB6];
        assert_eq!(opcode.mode, ZeroPageY);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0b0000_1100;
        cpu.pc = 0x0800;
        cpu.y = 0x01;
        bus.write_u8(cpu.pc + 1, 0x10);
        bus.write_u8(0x11, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0b0000_1010);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0xAE];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0b0000_1100;
        cpu.pc = 0x0800;
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1234, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0b0000_1010);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn absolute_y() {
        let opcode = OPTABLE[0xBE];
        assert_eq!(opcode.mode, AbsoluteY);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no page cross
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0b0000_1100;
        cpu.y = 0x1;
        cpu.pc = 0x0800;        
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1235, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.x, 0b0000_1010);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);

        // page cross (additional cycle)
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.x = 0b0000_1100;
        cpu.y = 0xff;
        cpu.pc = 0x0800;        
        bus.write_u16(cpu.pc + 1, 0x1234);
        bus.write_u8(0x1333, 0b0000_1010);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1);
        assert_eq!(cpu.x, 0b0000_1010);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x0803);
        assert_eq!(cpu.sp, 0xff);
    }
}