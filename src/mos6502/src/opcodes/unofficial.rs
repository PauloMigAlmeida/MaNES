use bus::Bus;
use super::{Mos6502, AddressingMode, Instruction};

/// IVL - Invalid Opcode
/// It's not my intention to implement unofficial opcodes at the moment
/// so I will simply panic the execution should I ever see one
pub fn invalid(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
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
        let (mut cpu, mut bus) = init();
        cpu.execute_instruction(0x02, &mut bus);
    }
}