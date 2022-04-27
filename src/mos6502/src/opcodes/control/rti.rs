use bus::Bus;
use crate::{Mos6502, Instruction};
use crate::Flags::{Break, Interrupt};

/// RTI - Return from Interrupt
///
/// The RTI instruction is used at the end of an interrupt processing routine.
/// It pulls the processor flags from the stack followed by the program counter.
pub fn rti(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.flags = cpu.stack_pull(bus);
    cpu.pc = cpu.stack_pull(bus) as u16 | (cpu.stack_pull(bus) as u16) << 8;
    cpu.clear_flag(Break);
    cpu.clear_flag(Interrupt);
    0
}

#[cfg(test)]
mod tests {
    use crate::Implicit;
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME: &str = "RTI";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn implicit() {
        let opcode = OPTABLE[0x40];
        assert_eq!(opcode.mode, Implicit);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xfc;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u16(0x01FE, 0x1234);
        bus.write_u8(0x01FD, 0b1000_0011);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b1000_0011);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.sp, 0xff);
    }

    // #[test]
    // fn test_rti() {
    //     let (mut cpu, mut bus) = init();
    //
    //     cpu.sp = 0xff;
    //     cpu.flags = 0b1100_1110;
    //     cpu.pc = 0x0203;
    //
    //     cpu.stack_push((cpu.pc & 0xff) as u8, &mut bus); // LSB
    //     cpu.stack_push(((cpu.pc & 0xff00) >> 8) as u8, &mut bus); //MSB
    //     cpu.stack_push(cpu.flags, &mut bus);
    //
    //     cpu.flags = 0;
    //     cpu.pc = 0;
    //
    //     let opcode = OPTABLE[0x40];
    //     let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
    //     assert_eq!(cycles, opcode.cycles);
    //     assert_eq!(cpu.flags, 0b1100_1110);
    //     assert_eq!(cpu.pc, 0x0203);
    //     assert_eq!(cpu.sp, 0xff);
    // }

}