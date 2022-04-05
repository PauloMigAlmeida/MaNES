mod control;
mod alu;
mod rmw;
mod unofficial;

use super::Mos6502;
use control::*;
use alu::*;
use rmw::*;
use unofficial::*;

type OpcodeFunction = fn(&mut Mos6502, AddressingMode);

pub struct Instruction {
    opcode: u8,
    name: String,
    cycles: u8,
    pub addressing_mode: AddressingMode,
    bytes: u8,
    page_cross_add_cycle: bool,
    pub function: OpcodeFunction,
}

#[derive(Debug)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

pub fn parse_instruction(opcode: u8) -> Instruction {
    match opcode {
        0x0 => Instruction {
            opcode,
            name: String::from("BRK"),
            cycles: 7,
            addressing_mode: AddressingMode::Implicit,
            bytes: 1,
            page_cross_add_cycle: false,
            function: brk,
        },
        0x1 => Instruction {
            opcode,
            name: String::from("ORA"),
            cycles: 6,
            addressing_mode: AddressingMode::IndirectX,
            bytes: 2,
            page_cross_add_cycle: false,
            function: brk,
        },
        0x2 => Instruction {
            opcode,
            name: String::from("NOP"),
            cycles: 2,
            addressing_mode: AddressingMode::Immediate,
            bytes: 1,
            page_cross_add_cycle: false,
            function: brk,
        },
        _ => panic!("opcode: {} is not valid", opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_brk() {
        let result = parse_instruction(0x0);
        assert_eq!(result.name, "BRK");
        
    }
}