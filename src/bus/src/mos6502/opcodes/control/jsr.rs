use super::*;

/// JSR - Jump to Subroutine
///
/// The JSR instruction pushes the address (minus one) of the return point on to the stack and
/// then sets the program counter to the target memory address.
pub fn jsr(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    // it's funny but it took me a while to figure out why on earth I have to decrement pc before
    // pushing it to the stack... It turns out that RTS instruction pull the PC from the stack and
    // the instruction size is added to PC which essentially brings it back to the original value.
    // Moral of the story, in source code that looks like a workaround but in hardware that was most
    // likely a way to reduce the complexity of hardware's function implementations.. funny world :)
    let old_pc = cpu.pc;
    cpu.pc += inst.bytes as u16 - 1;
    cpu.stack_push(((cpu.pc >> 8) & 0x00FF) as u8, bus);
    cpu.stack_push((cpu.pc & 0x00FF) as u8, bus);
    cpu.pc = bus.cpu_read_u16(old_pc + 1, false);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME: &str = "JSR";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0x20];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0011;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.cpu_write_u16(0x0801, 0x1234);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0011);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(bus.cpu_read_u16(0x01FE, false), 0x0802);
    }
}