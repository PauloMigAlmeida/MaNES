use bus::Bus;
use crate::{Mos6502, Instruction};
use crate::{Absolute, Indirect};

/// JMP - Jump
///
/// Sets the program counter to the address specified by the operand.
pub fn jmp(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);

    let addr = match inst.mode {
        Absolute => {
            bus.read_u16(cpu.pc + 1)
        },
        Indirect => {
            let addr_ptr = bus.read_u16(cpu.pc + 1);
            let addr_abs:u16;

            if (addr_ptr & 0xFF) == 0xFF {
                // Simulate page boundary hardware bug
                addr_abs = ((bus.read_u8(addr_ptr & 0xFF00) as u16) << 8)  | bus.read_u8(addr_ptr) as u16;
            }else {
                // Behave normally
                addr_abs = bus.read_u16(addr_ptr);
            }
            addr_abs
        },
        _ => panic!("invalid addressing mode for instruction")
    };

    cpu.pc = addr;
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::{OPTABLE};

    const OPCODE_NAME: &str = "JMP";

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn absolute() {
        let opcode = OPTABLE[0x4C];
        assert_eq!(opcode.mode, Absolute);
        assert_eq!(opcode.name, OPCODE_NAME);

        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0011;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u16(0x0801, 0x1234);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0011);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn indirect() {
        let opcode = OPTABLE[0x6C];
        assert_eq!(opcode.mode, Indirect);
        assert_eq!(opcode.name, OPCODE_NAME);

        // behave normally
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0011;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u16(0x0801, 0x1234);
        bus.write_u16(0x1234, 0x0200);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0011);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0200);
        assert_eq!(cpu.sp, 0xff);

        //mimic hardware bug
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.pc = 0x0800;
        cpu.flags = 0b0000_0011;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        bus.write_u16(0x0801, 0x12FF);
        bus.write_u16(0x12FF, 0x00);
        bus.write_u16(0x1200, 0x02);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.flags, 0b0000_0011);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0200);
        assert_eq!(cpu.sp, 0xff);
    }
}