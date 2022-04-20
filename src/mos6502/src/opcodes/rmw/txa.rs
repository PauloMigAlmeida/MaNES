use bus::Bus;
use crate::{Mos6502, Instruction, Flags::*};

/// TXA - Transfer X to Accumulator
/// A = X
///
/// Copies the current contents of the X register into the accumulator 
/// and sets the zero and negative flags as appropriate.
pub fn txa(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.a = cpu.x;
    cpu.write_flag_cond(Zero, cpu.a == 0);
    cpu.write_flag_cond(Negative, cpu.a & 0x80 == 0x80);
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests {
    use crate::{Implicit};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME: &str = "TXA";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0x8A];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        // no flags set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0x10;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.x, 0x10);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);

        // negative flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0x80;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.x, 0x80);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);

        // zero flag set
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0x10;
        cpu.x = 0;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);
        
    }
}