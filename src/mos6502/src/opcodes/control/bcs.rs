use bus::Bus;
use crate::{Mos6502, Instruction, Flags::*};

/// BCS - Branch if Carry Set
/// If the Carry flag is set then add the relative displacement to the program counter to
/// cause a branch to a new location.
pub fn bcs(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let (fetched, _) = cpu.address_mode_fetch(bus, &inst);
    let mut additional_cycles = 0;

    if cpu.is_flag_set(Carry) {
        let mut fetched = fetched as u16;
        if fetched & 0x80 == 0x80 {
            fetched |= 0xFF00;
        }
        let target_addr = cpu.pc.overflowing_add(fetched).0;

        // check whether we crossed a page
        if (cpu.pc & 0xff00) != (target_addr & 0xff00) {
            additional_cycles = 2;
        }else {
            additional_cycles = 1;
        }

        cpu.pc = target_addr;
    } else {
        cpu.pc += inst.bytes as u16;
    }

    additional_cycles
}

#[cfg(test)]
mod tests {
    use crate::{Relative};
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME: &str = "BCS";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn relative() {
        let opcode = OPTABLE[0xB0];
        assert_eq!(opcode.mode, Relative);
        assert_eq!(opcode.name, OPCODE_NAME);

        // Not branch
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0802);
        assert_eq!(cpu.sp, 0xff);

        // branch -> pc increment
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0001;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1); // branch to the same page cost 1 additional cycle
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0810);
        assert_eq!(cpu.sp, 0xff);

        // branch -> pc increment -> different page
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x08f0;
        cpu.flags = 0b0000_0001;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0x10);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 2); // branch to the new page cost 2 additional cycles
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0900);
        assert_eq!(cpu.sp, 0xff);

        // branch -> pc decrement
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x08f0;
        cpu.flags = 0b0000_0001;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0xfa); // -6 two's complement
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 1); // branch to the same page cost 1 additional cycle
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x08ea);
        assert_eq!(cpu.sp, 0xff);

        // branch -> pc decrement -> diff page
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0001;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u8(cpu.pc + 1, 0xfa); // -6 two's complement
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles + 2); // branch to diff page cost 2 additional cycles
        assert_eq!(cpu.flags, 0b0000_0001);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x07fa);
        assert_eq!(cpu.sp, 0xff);
    }

}