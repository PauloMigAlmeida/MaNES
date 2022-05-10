use super::*;

/// BRK - Force Interrupt
///
/// The BRK instruction forces the generation of an interrupt request.
/// The program counter and processor status are pushed on the stack then the
/// IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag
/// in the status set to one.
pub fn brk(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.pc += inst.bytes as u16;

    cpu.set_flag(DisableInterrupt);
    cpu.stack_push((cpu.pc >> 8) as u8, bus);
    cpu.stack_push((cpu.pc & 0x00FF) as u8, bus);
    cpu.set_flag(Break);
    cpu.stack_push(cpu.flags, bus);
    cpu.clear_flag(Break);

    cpu.pc = bus.read_u16(0xFFFE);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPCODE_NAME: &str = "BRK";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    // TODO implement vectors otherise this test will fail
    //
    // #[test]
    // fn implicit() {
    //     let opcode = OPTABLE[0x00];
    //     assert_eq!(opcode.mode, Implicit);
    //     assert_eq!(opcode.name, OPCODE_NAME);
    //
    //     let (mut cpu, mut bus) = init();
    //     cpu.sp = 0xff;
    //     cpu.pc = 0x0800;
    //     cpu.flags = 0b0000_0011;
    //     cpu.a = 0;
    //     cpu.x = 0;
    //     cpu.y = 0;
    //     bus.write_u16(0xFFFE, 0x1234);
    //     let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
    //     assert_eq!(cycles, opcode.cycles);
    //     assert_eq!(cpu.flags, 0b0000_0111);
    //     assert_eq!(cpu.a, 0);
    //     assert_eq!(cpu.x, 0);
    //     assert_eq!(cpu.y, 0);
    //     assert_eq!(cpu.pc, 0x1234);
    //     assert_eq!(cpu.sp, 0xfc);
    //     assert_eq!(bus.read_u16(0xFFFE), 0x1234); // check memory is intact
    //     assert_eq!(bus.read_u16(0x01FE), 0x0801); // check right pc is pushed to stack
    //     assert_eq!(bus.read_u8(0x01FD), 0b0001_0111); // check right pc is pushed to stack
    // }
}