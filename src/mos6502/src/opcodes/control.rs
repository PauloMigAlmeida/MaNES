use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scafold 

/// BRK - Force Interrupt
/// The BRK instruction forces the generation of an interrupt request. 
/// The program counter and processor status are pushed on the stack then the 
/// IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag 
/// in the status set to one.
pub fn brk(cpu: &mut Mos6502, addr_mode: AddressingMode) {
    println!("brk was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brk() {
        let mut cpu = Mos6502::new();
        cpu.execute_instruction(0x00);
        assert_eq!(cpu.reg_a, 0x00);
    }
}