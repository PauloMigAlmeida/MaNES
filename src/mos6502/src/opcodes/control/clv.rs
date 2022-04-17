use bus::Bus;
use super::Mos6502;
use super::Instruction;
use crate::Flags::*;

/// CLV - Clear Overflow Flag
/// Clears the overflow flag.
pub fn clv(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.clear_flag(Overflow);
    cpu.pc += 1;
    0
}

#[cfg(test)]
mod tests {
    use crate::{Implicit};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME: &str = "CLV";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0xb8];
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
        assert_eq!(cpu.flags, 0b1000_1111);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);
    }
}