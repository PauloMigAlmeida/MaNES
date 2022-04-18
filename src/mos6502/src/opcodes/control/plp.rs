use bus::Bus;
use crate::{Mos6502, Instruction};

/// PLP - Pull Processor Status
/// Pulls an 8 bit value from the stack and into the processor flags.
/// The flags will take on new states as determined by the value pulled.
pub fn plp(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let value = cpu.stack_pull(bus);
    cpu.flags = value;
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests {
    use crate::{Implicit};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME:&str = "PLP";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0x28];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        bus.write_u8(0x01ff, 0b1100_1111);
        cpu.sp = 0xfe;
        cpu.pc = 0x0800;
        cpu.flags = 0;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b1100_1111);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);
    }
}