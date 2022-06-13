use super::*;

/// RTS - Return from Subroutine
///
/// The RTS instruction is used at the end of a subroutine to return to the calling routine.
/// It pulls the program counter (minus one) from the stack.
pub fn rts(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.pc = cpu.stack_pull(bus) as u16 | (cpu.stack_pull(bus) as u16) << 8;
    cpu.pc += inst.bytes as u16;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME: &str = "RTS";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0x60];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xfd;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0011;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.cpu_write_u16(0x01FE, 0x1234);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0011);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x1235);
        assert_eq!(cpu.sp, 0xff);
    }

}