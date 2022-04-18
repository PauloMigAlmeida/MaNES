use bus::Bus;
use crate::{Mos6502, Instruction, Flags::*};

/// DEY - Decrement Y Register
/// Y,Z,N = Y-1
///
/// Subtracts one from the Y register setting the zero and negative flags as appropriate.
pub fn dey(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.y = cpu.y.overflowing_sub(1).0;
    cpu.write_flag_cond(Zero, cpu.y == 0);
    cpu.write_flag_cond(Negative, cpu.y & 0x80 == 0x80);
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests{
    use crate::{Implicit};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME:&str = "DEY";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0x88];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0b0000_0010;
        cpu.pc = 0x10;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x11);
        assert_eq!(cpu.sp, 0xff);

        // test zero, negative flags clear
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b1000_0010;
        cpu.y = 0b0000_0010;
        cpu.pc = 0x10;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0b0000_0001);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x11);
        assert_eq!(cpu.sp, 0xff);

        // zero flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0000;
        cpu.y = 0b0000_0001;
        cpu.pc = 0x10;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0b0000_0000);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.pc, 0x11);
        assert_eq!(cpu.sp, 0xff);

        // negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b0000_0010;
        cpu.y = 0b0000_0000;
        cpu.pc = 0x10;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0b1111_1111); // -1 two's complement
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x11);
        assert_eq!(cpu.sp, 0xff);

        // negative flag set, decrement negative numbers
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b1000_0000;
        cpu.y = 0b1111_0110; // -10 two's complement
        cpu.pc = 0x10;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.y, 0b1111_0101); // -11 two's complement);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x11);
        assert_eq!(cpu.sp, 0xff);
    }
    
}