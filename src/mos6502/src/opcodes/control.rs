use bus::Bus;
use super::Flags::*;
use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scaffold

/// BRK - Force Interrupt
/// The BRK instruction forces the generation of an interrupt request. 
/// The program counter and processor status are pushed on the stack then the 
/// IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag 
/// in the status set to one.
pub fn brk(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("brk was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// PHP - Push Processor Status
/// Pushes a copy of the status flags on to the stack.
pub fn php(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("php was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.stack_push(cpu.flags, bus);
    0
}

pub fn bpl(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("bpl was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// CLC - Clear Carry Flag
/// Set the carry flag to zero.
pub fn clc(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &mut Bus) -> u8 {
    println!("clc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.clear_flag(Carry);
    0
}

pub fn jmp(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("jmp was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// CLI - Clear Interrupt Disable
/// Clears the interrupt disable flag allowing normal interrupt
/// requests to be serviced.
pub fn cli(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &mut Bus) -> u8 {
    println!("cli was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.clear_flag(Interrupt);
    0
}

/// SEI - Set Interrupt Disable
/// Set the interrupt disable flag to one.
pub fn sei(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &mut Bus) -> u8 {
    println!("sei was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.set_flag(Interrupt);
    0
}

pub fn bvs(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("bvs was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn jsr(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("jsr was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bvc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("bvc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn rts(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("rts was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// PLA - Pull Accumulator
/// Pulls an 8 bit value from the stack and into the accumulator. 
/// The zero and negative flags are set as appropriate.
pub fn pla(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("pla was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    let value = cpu.stack_pull(bus);
    if value == 0 {
        cpu.set_flag(Zero);
    }
    if (value & (1 << 7)) != 0 {
        cpu.set_flag(Negative);
    }
    cpu.a = value;
    0
}

/// PHA - Push Accumulator
/// Pushes a copy of the accumulator on to the stack.
pub fn pha(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("pha was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.stack_push(cpu.a, bus);
    0
}

/// RTI - Return from Interrupt
/// The RTI instruction is used at the end of an interrupt processing routine. It pulls the processor flags from the stack followed by the program counter.
pub fn rti(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("rti was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.flags = cpu.stack_pull(bus);

    //TODO verify if that's the order in which I put PC value back (you know, endian vs little-endian still drives me nuts)
    let mut value = cpu.stack_pull(bus) as u16;
    value <<= 8;
    value |= cpu.stack_pull(bus) as u16;
    cpu.pc = value;

    0
}

/// SEC - Set Carry Flag
/// Set the carry flag to one.
pub fn sec(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &mut Bus) -> u8 {
    println!("sec was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.set_flag(Carry);
    0
}

pub fn bit(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("bit was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn plp(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("plp was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    let value = cpu.stack_pull(bus);
    cpu.flags = value;
    0
}

pub fn bmi(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("bmi was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn sty(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("sty was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn dey(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("dey was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bcc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("bcc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn tya(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("tya was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn ldy(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("ldy was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn tay(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("tay was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bcs(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("bcs was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// CLV - Clear Overflow Flag
/// Clears the overflow flag.
pub fn clv(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &mut Bus) -> u8 {
    println!("clv was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.clear_flag(Overflow);
    0
}

pub fn cpy(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("cpy was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn iny(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("iny was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn bne(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("bne was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// CLD - Clear Decimal Mode
/// Sets the decimal mode flag to zero.
pub fn cld(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &mut Bus) -> u8 {
    println!("cld was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.clear_flag(Decimal);
    0
}

pub fn cpx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("cpx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn inx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("inx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn beq(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("beq was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

/// SED - Set Decimal Flag
/// Set the decimal mode flag to one.
pub fn sed(cpu: &mut Mos6502, addr_mode: AddressingMode, _bus: &mut Bus) -> u8 {
    println!("sed was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    cpu.set_flag(Decimal);
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
    fn test_brk() {
        let (mut cpu, mut bus) = init();
        cpu.execute_instruction(0x00, &mut bus);
        assert_eq!(cpu.a, 0x00);
    }

    #[test]
    fn test_cli() {
        let (mut cpu, mut bus) = init();        
        cpu.flags = 0b1100_1111;
        common_execute(&mut cpu, &mut bus, 0x58);
        assert_eq!(cpu.flags, 0b1100_1011);        
    }

    #[test]
    fn test_sei() {
        let (mut cpu, mut bus) = init();        
        cpu.flags = 0b1100_1011;
        common_execute(&mut cpu, &mut bus, 0x78);
        assert_eq!(cpu.flags, 0b1100_1111);        
    }

    #[test]
    fn test_clc() {
        let (mut cpu, mut bus) = init();        
        cpu.flags = 0b1100_1111;
        common_execute(&mut cpu, &mut bus, 0x18);
        assert_eq!(cpu.flags, 0b1100_1110);
    }

    #[test]
    fn test_sec() {
        let (mut cpu, mut bus) = init();        
        cpu.flags = 0b1100_1110;
        common_execute(&mut cpu, &mut bus, 0x38);
        assert_eq!(cpu.flags, 0b1100_1111);
    }

    #[test]
    fn test_clv() {
        let (mut cpu, mut bus) = init();        
        cpu.flags = 0b1100_1111;
        common_execute(&mut cpu, &mut bus, 0xb8);
        assert_eq!(cpu.flags, 0b1000_1111);
    }

    #[test]
    fn test_cld() {
        let (mut cpu, mut bus) = init();        
        cpu.flags = 0b1100_1111;
        common_execute(&mut cpu, &mut bus, 0xd8);
        assert_eq!(cpu.flags, 0b1100_0111);
    }

    #[test]
    fn test_sed() {
        let (mut cpu, mut bus) = init();        
        cpu.flags = 0b1100_0111;
        common_execute(&mut cpu, &mut bus, 0xf8);
        assert_eq!(cpu.flags, 0b1100_1111);
    }

    #[test]
    fn test_pha() {
        let (mut cpu, mut bus) = init();        
        cpu.sp = 0xff;
        cpu.a = 0x10;
        common_execute(&mut cpu, &mut bus, 0x48);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(bus.read_address(0x01ff), 0x10);
    }

    #[test]
    fn test_pla() {
        let (mut cpu, mut bus) = init();        
        cpu.sp = 0xff;

        cpu.a = 0x10;
        common_execute(&mut cpu, &mut bus, 0x48); // push
        cpu.a = 0x11; 
        common_execute(&mut cpu, &mut bus, 0x68); // pull        
        assert_eq!(cpu.a, 0x10); // should override accumulator
        assert_eq!(cpu.sp, 0xff);

        // Test zero flag
        cpu.flags = 0b0000_0000;
        cpu.a = 0x0;
        common_execute(&mut cpu, &mut bus, 0x48);
        cpu.a = 0x1; 
        common_execute(&mut cpu, &mut bus, 0x68); 
        assert_eq!(cpu.a, 0x0); 
        assert_eq!(cpu.flags, 0b0000_0010);
        assert_eq!(cpu.sp, 0xff);

        // Test negative flag
        cpu.flags = 0b0000_0000;
        cpu.a = 0xff;
        common_execute(&mut cpu, &mut bus, 0x48);
        cpu.a = 0x1; 
        common_execute(&mut cpu, &mut bus, 0x68); 
        assert_eq!(cpu.a, 0xff); 
        assert_eq!(cpu.flags, 0b1000_0000);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn test_php() {
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;
        cpu.flags = 0b1100_1110;
        common_execute(&mut cpu, &mut bus, 0x08);
        assert_eq!(cpu.flags, 0b1100_1110);
        assert_eq!(bus.read_address(0x01ff), 0b1100_1110);
    }

    #[test]
    fn test_plp() {
        let (mut cpu, mut bus) = init();
        cpu.sp = 0xff;

        cpu.flags = 0b1100_1110;
        common_execute(&mut cpu, &mut bus, 0x08); // php
        cpu.flags = 0;
        common_execute(&mut cpu, &mut bus, 0x28); //plp
        assert_eq!(cpu.flags, 0b1100_1110);
        assert_eq!(cpu.sp, 0xff);
    }
}