use bus::Bus;
use super::Flags::*;
use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scaffold

/// BRK - Force Interrupt
/// The BRK instruction forces the generation of an interrupt request. 
/// The program counter and processor status are pushed on the stack then the 
/// IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag 
/// in the status set to one.
pub fn brk(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("brk was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn php(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("php was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bpl(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("bpl was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// CLC - Clear Carry Flag
/// Set the carry flag to zero.
pub fn clc(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &Bus) -> u8 {
    println!("clc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.clear_flag(Carry);
    0
}

pub fn jmp(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("jmp was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// CLI - Clear Interrupt Disable
/// Clears the interrupt disable flag allowing normal interrupt
/// requests to be serviced.
pub fn cli(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &Bus) -> u8 {
    println!("cli was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.clear_flag(Interrupt);
    0
}

/// SEI - Set Interrupt Disable
/// Set the interrupt disable flag to one.
pub fn sei(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &Bus) -> u8 {
    println!("sei was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.set_flag(Interrupt);
    0
}

pub fn bvs(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("bvs was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn jsr(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("jsr was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bvc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("bvc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn rts(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("rts was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn pla(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("pla was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn pha(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("pha was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn rti(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("rti was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// SEC - Set Carry Flag
/// Set the carry flag to one.
pub fn sec(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &Bus) -> u8 {
    println!("sec was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.set_flag(Carry);
    0
}

pub fn bit(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("bit was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn plp(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("plp was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bmi(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("bmi was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn sty(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("sty was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn dey(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("dey was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bcc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("bcc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn tya(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("tya was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn ldy(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("ldy was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn tay(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("tay was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bcs(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("bcs was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// CLV - Clear Overflow Flag
/// Clears the overflow flag.
pub fn clv(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &Bus) -> u8 {
    println!("clv was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.clear_flag(Overflow);
    0
}

pub fn cpy(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("cpy was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn iny(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("iny was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bne(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("bne was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// CLD - Clear Decimal Mode
/// Sets the decimal mode flag to zero.
pub fn cld(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &Bus) -> u8 {
    println!("cld was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.clear_flag(Decimal);
    0
}

pub fn cpx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("cpx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn inx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("inx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn beq(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) -> u8 {
    println!("beq was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// SED - Set Decimal Flag
/// Set the decimal mode flag to one.
pub fn sed(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &Bus) -> u8 {
    println!("sed was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.set_flag(Decimal);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::OPTABLE;

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn test_brk() {
        let (mut cpu, bus) = init();
        cpu.execute_instruction(0x00, &bus);
        assert_eq!(cpu.a, 0x00);
    }

    #[test]
    fn test_cli() {
        let (mut cpu, bus) = init();        
        let opcode = OPTABLE[0x58];

        cpu.flags = 0b1100_1111;
        let cycles = cpu.execute_instruction(opcode.opcode, &bus);
        assert_eq!(cpu.flags, 0b1100_1011);
        assert_eq!(cycles, opcode.cycles);
    }

    #[test]
    fn test_sei() {
        let (mut cpu, bus) = init();        
        let opcode = OPTABLE[0x78];

        cpu.flags = 0b1100_1011;
        let cycles = cpu.execute_instruction(opcode.opcode, &bus);
        assert_eq!(cpu.flags, 0b1100_1111);
        assert_eq!(cycles, opcode.cycles);
    }

    #[test]
    fn test_clc() {
        let (mut cpu, bus) = init();        
        let opcode = OPTABLE[0x18];

        cpu.flags = 0b1100_1111;
        let cycles = cpu.execute_instruction(opcode.opcode, &bus);
        assert_eq!(cpu.flags, 0b1100_1110);
        assert_eq!(cycles, opcode.cycles);
    }

    #[test]
    fn test_sec() {
        let (mut cpu, bus) = init();        
        let opcode = OPTABLE[0x38];

        cpu.flags = 0b1100_1110;
        let cycles = cpu.execute_instruction(opcode.opcode, &bus);
        assert_eq!(cpu.flags, 0b1100_1111);
        assert_eq!(cycles, opcode.cycles);
    }

    #[test]
    fn test_clv() {
        let (mut cpu, bus) = init();        
        let opcode = OPTABLE[0xb8];

        cpu.flags = 0b1100_1111;
        let cycles = cpu.execute_instruction(opcode.opcode, &bus);
        assert_eq!(cpu.flags, 0b1000_1111);
        assert_eq!(cycles, opcode.cycles);
    }

    #[test]
    fn test_cld() {
        let (mut cpu, bus) = init();        
        let opcode = OPTABLE[0xd8];

        cpu.flags = 0b1100_1111;
        let cycles = cpu.execute_instruction(opcode.opcode, &bus);
        assert_eq!(cpu.flags, 0b1100_0111);
        assert_eq!(cycles, opcode.cycles);
    }

    #[test]
    fn test_sed() {
        let (mut cpu, bus) = init();        
        let opcode = OPTABLE[0xf8];

        cpu.flags = 0b1100_0111;
        let cycles = cpu.execute_instruction(opcode.opcode, &bus);
        assert_eq!(cpu.flags, 0b1100_1111);
        assert_eq!(cycles, opcode.cycles);
    }
}