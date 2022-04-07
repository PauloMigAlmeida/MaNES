use bus::Bus;
use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scaffold

/// BRK - Force Interrupt
/// The BRK instruction forces the generation of an interrupt request. 
/// The program counter and processor status are pushed on the stack then the 
/// IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag 
/// in the status set to one.
pub fn brk(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("brk was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn php(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("php was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn bpl(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("bpl was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn clc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("clc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn jmp(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("jmp was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn cli(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("cli was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn sei(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("sei was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn bvs(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("bvs was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn jsr(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("jsr was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn bvc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("bvc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn rts(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("rts was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn pla(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("pla was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn pha(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("pha was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn rti(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("rti was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn sec(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("sec was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn bit(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("bit was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn plp(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("plp was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn bmi(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("bmi was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn test_brk() {
        let (mut cpu, bus) = init();
        cpu.execute_instruction(0x00, &bus);
        assert_eq!(cpu.a, 0x00);
    }
}