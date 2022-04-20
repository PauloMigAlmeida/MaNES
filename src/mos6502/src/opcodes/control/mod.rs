mod cli;
mod sei;
mod clc;
mod sec;
mod clv;
mod cld;
mod sed;
mod sty;
mod pha;
mod pla;
mod php;
mod plp;
mod dey;
mod ldy;
mod bpl;
mod bvs;
mod bvc;
mod bcc;
mod bcs;
mod beq;
mod bmi;
mod bne;
mod bit;
use bus::Bus;
use super::{Mos6502, Instruction};
pub use cli::*;
pub use sei::*;
pub use clc::*;
pub use sec::*;
pub use clv::*;
pub use cld::*;
pub use sed::*;
pub use sty::*;
pub use pha::*;
pub use pla::*;
pub use php::*;
pub use plp::*;
pub use dey::*;
pub use ldy::*;
pub use bpl::*;
pub use bvs::*;
pub use bvc::*;
pub use bcc::*;
pub use bcs::*;
pub use beq::*;
pub use bmi::*;
pub use bne::*;
pub use bit::*;

//TODO implement actual functions here... right now I'm just interested in the scaffold

/// BRK - Force Interrupt
/// The BRK instruction forces the generation of an interrupt request. 
/// The program counter and processor status are pushed on the stack then the 
/// IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag 
/// in the status set to one.
pub fn brk(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn jmp(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn jsr(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

/// RTS - Return from Subroutine
/// The RTS instruction is used at the end of a subroutine to return to the calling routine. 
/// It pulls the program counter (minus one) from the stack.
pub fn rts(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    
    //TODO check if I really need to decrement pc here... my initial mental model says that I don't while the docs says
    // that I do... I guess I will learn when things break magestically :)    
    let mut value = cpu.stack_pull(bus) as u16;
    value <<= 8;
    value |= cpu.stack_pull(bus) as u16;
    cpu.pc = value;

    0
}

/// RTI - Return from Interrupt
/// The RTI instruction is used at the end of an interrupt processing routine. It pulls the processor flags from the stack followed by the program counter.
pub fn rti(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.flags = cpu.stack_pull(bus);

    // TODO verify if that's the order in which I put PC value back (you know, endian vs little-endian still drives me nuts)
    // Also, I'm not 100% convinced that I should just to replace cpu.pc with the value out of the stack as the pc value gets 
    // incremented with the instruction size (in this case 1)... 
    let mut value = cpu.stack_pull(bus) as u16;
    value <<= 8;
    value |= cpu.stack_pull(bus) as u16;
    cpu.pc = value;

    0
}

pub fn tya(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn tay(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn cpy(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn iny(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn cpx(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn inx(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::{OPTABLE};

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    fn test_rti() {
        let (mut cpu, mut bus) = init();
        
        cpu.sp = 0xff;        
        cpu.flags = 0b1100_1110;
        cpu.pc = 0x0203;

        cpu.stack_push((cpu.pc & 0xff) as u8, &mut bus); // LSB
        cpu.stack_push(((cpu.pc & 0xff00) >> 8) as u8, &mut bus); //MSB
        cpu.stack_push(cpu.flags, &mut bus);
        
        cpu.flags = 0;
        cpu.pc = 0;

        let opcode = OPTABLE[0x40];        
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);        
        assert_eq!(cpu.flags, 0b1100_1110);
        assert_eq!(cpu.pc, 0x0203);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn test_rts() {
        let (mut cpu, mut bus) = init();
        
        cpu.sp = 0xff;        
        cpu.flags = 1;
        cpu.pc = 0x0203;

        cpu.stack_push((cpu.pc & 0xff) as u8, &mut bus); // LSB
        cpu.stack_push(((cpu.pc & 0xff00) >> 8) as u8, &mut bus); //MSB
        
        cpu.pc = 0;

        let opcode = OPTABLE[0x60];        
        let cycles = cpu.execute_instruction(opcode.opcode, &mut bus);
        assert_eq!(cycles, opcode.cycles);        
        assert_eq!(cpu.flags, 1);
        assert_eq!(cpu.pc, 0x0203);
        assert_eq!(cpu.sp, 0xff);
    }
}