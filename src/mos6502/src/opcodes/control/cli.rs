use bus::Bus;
use crate::{Mos6502, Instruction, Flags::*};

/// CLI - Clear Interrupt Disable
/// Clears the interrupt disable flag allowing normal interrupt
/// requests to be serviced.
pub fn cli(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.clear_flag(Interrupt);
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests {
    use crate::{Implicit};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME: &str = "CLI";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0x58];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b1100_1111;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b1100_1011);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);
    }
}