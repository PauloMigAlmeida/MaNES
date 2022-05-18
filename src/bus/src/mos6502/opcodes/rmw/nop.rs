use super::*;

/// NOP - No Operation
/// The NOP instruction causes no changes to the processor other than the normal incrementing
/// of the program counter to the next instruction.
pub fn nop(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.pc += 1;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME:&str = "NOP";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0xEA];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0801);
        assert_eq!(cpu.sp, 0xff);
    }
}