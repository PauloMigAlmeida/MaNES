use bus::Bus;
use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn asl(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("asl was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn rol(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("rol was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn ror(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("ror was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn lsr(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("lsr was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn stx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("stx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn txa(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("tax was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn txs(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("txs was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn ldx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("ldx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn tax(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("tax was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn tsx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("tsx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn dec(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("dec was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn dex(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("dex was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn inc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("dex was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// NOP - No Operation
/// The NOP instruction causes no changes to the processor other than the normal incrementing
/// of the program counter to the next instruction.
pub fn nop(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &mut Bus) -> u8 {
    println!("nop was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::{OPTABLE};

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    fn common_execute(cpu: &mut Mos6502, bus: &mut Bus, opcode: usize) {
        let opcode = OPTABLE[opcode];
        let old_pc = cpu.pc;
        let cycles = cpu.execute_instruction(opcode.opcode, bus);
        assert_eq!(cycles, opcode.cycles);
        assert_eq!(old_pc + opcode.bytes as u16, cpu.pc);
    }

    #[test]
    fn test_nop() {
        let (mut cpu, mut bus) = init();
        cpu.flags = 0b0000_0000;
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        common_execute(&mut cpu, &mut bus, 0xea);
        assert_eq!(cpu.flags, 0b0000_0000);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
    }
}