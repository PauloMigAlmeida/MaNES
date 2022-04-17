mod cli;
mod sei;
mod clc;
mod sec;
mod clv;
mod cld;
mod sed;
use bus::Bus;
use super::Flags::*;
use super::{Mos6502, Instruction};
pub use cli::*;
pub use sei::*;
pub use clc::*;
pub use sec::*;
pub use clv::*;
pub use cld::*;
pub use sed::*;

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

/// PHP - Push Processor Status
/// Pushes a copy of the status flags on to the stack.
pub fn php(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.stack_push(cpu.flags, bus);
    cpu.pc += 1;
    0
}

pub fn bpl(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn jmp(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn bvs(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn jsr(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn bvc(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
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

/// PLA - Pull Accumulator
/// Pulls an 8 bit value from the stack and into the accumulator. 
/// The zero and negative flags are set as appropriate.
pub fn pla(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let value = cpu.stack_pull(bus);
    if value == 0 {
        cpu.set_flag(Zero);
    }
    if (value & (1 << 7)) != 0 {
        cpu.set_flag(Negative);
    }
    cpu.a = value;
    cpu.pc += 1;
    0
}

/// PHA - Push Accumulator
/// Pushes a copy of the accumulator on to the stack.
pub fn pha(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    cpu.stack_push(cpu.a, bus);
    cpu.pc += 1;
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

pub fn bit(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn plp(cpu: &mut Mos6502, inst: Instruction, bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    let value = cpu.stack_pull(bus);
    cpu.flags = value;
    cpu.pc += 1;
    0
}

pub fn bmi(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn sty(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn dey(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn bcc(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn tya(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn ldy(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn tay(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn bcs(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
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

pub fn bne(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
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

pub fn beq(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
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
    fn test_pha() {
        let (mut cpu, mut bus) = init();        
        cpu.sp = 0xff;
        cpu.a = 0x10;
        common_execute(&mut cpu, &mut bus, 0x48);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(bus.read_u8(0x01ff), 0x10);
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
        assert_eq!(bus.read_u8(0x01ff), 0b1100_1110);
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