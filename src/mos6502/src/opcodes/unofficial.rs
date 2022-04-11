use bus::Bus;
use super::{Mos6502, AddressingMode};

/// IVL - Invalid Opcode
/// It's not my intention to implement unofficial opcodes at this moment
/// so I will simply panic the execution should I ever see one
pub fn invalid(_cpu: &mut Mos6502, _addr_mode: AddressingMode, _bus: &Bus) -> u8 {
    panic!("Invalid opcode found... aborting");
}


#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> (Mos6502, Bus) {
        (Mos6502::new(), Bus::new())
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
        let (mut cpu, bus) = init();
        cpu.execute_instruction(0x02, &bus);
    }
}