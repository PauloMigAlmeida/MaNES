use bus::Bus;
use crate::Flags::{Negative, Zero};
use super::{Mos6502, AddressingMode::*, Instruction};

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn ora(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn adc(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn and(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);

    let fetched= match inst.mode {
        Immediate => bus.read_address(cpu.pc + 1),
        _ => panic!("not implemented yet"),
    };

    cpu.a = cpu.a & fetched;

    if cpu.a == 0 {
        cpu.set_flag(Zero);
    }

    if cpu.a & 0x80 == 0x80 {
        cpu.set_flag(Negative);
    }
    cpu.pc += inst.bytes as u16;
    0
}

pub fn eor(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn sta(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn lda(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn cmp(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn sbc(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

#[cfg(test)]
mod test{

    use super::*;
    use crate::opcodes::{OPTABLE};

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn test_and() {
        let (mut cpu, mut bus) = init();
        let opcode = OPTABLE[0x29];
        cpu.sp = 0xff;

        // Immediate mode, no flags set
        cpu.flags = 0b0000_0000;
        cpu.a = 0b0000_1100;
        cpu.pc = 0x10;
        bus.write_address(cpu.pc + 1, 0b0000_1000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b0000_1000);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);

        // Immediate mode, zero flag set
        cpu.flags = 0b0000_0000;
        cpu.a = 0b1111_1111;
        cpu.pc = 0x10;
        bus.write_address(cpu.pc + 1, 0b1000_0000);
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(cpu.a, 0b1000_0000);
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.pc, 0x12);
        assert_eq!(cpu.sp, 0xff);
    }
}