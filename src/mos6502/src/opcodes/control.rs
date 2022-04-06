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