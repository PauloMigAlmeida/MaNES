use bus::Bus;
use crate::{Mos6502, Instruction};

/// TXS - Transfer X to Stack Pointer
/// S = X
///
/// Copies the current contents of the X register into the stack register.
pub fn txs(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.sp = cpu.x;
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests {
    use crate::{Implicit};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME: &str = "TXS";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0x9A];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0x10;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0xff;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0xff);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);
        
    }
}