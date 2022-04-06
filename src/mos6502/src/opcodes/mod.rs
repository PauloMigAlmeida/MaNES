mod control;
mod alu;
mod rmw;
mod unofficial;

use super::Mos6502;
use control::*;
use alu::*;
use bus::Bus;
use rmw::*;
use unofficial::*;
use AddressingMode::*;

type OpcodeFunction = fn(&mut Mos6502, AddressingMode, &Bus);

#[derive(Copy, Clone)]
pub struct Instruction<'a> {
    opcode: u8,
    name: &'a str,
    cycles: u8,
    pub addressing_mode: AddressingMode,
    bytes: u8,
    pub function: OpcodeFunction,
}

#[derive(Debug, Copy, Clone)]
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

static OPTABLE: [Instruction; 256] = [
    // 0
    Instruction { opcode: 0x00, name: "BRK", cycles: 7, addressing_mode: Implicit,    bytes: 1, function: brk },
    Instruction { opcode: 0x01, name: "ORA", cycles: 6, addressing_mode: IndirectX,   bytes: 2, function: ora },
    Instruction { opcode: 0x02, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x03, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x04, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x05, name: "ORA", cycles: 3, addressing_mode: ZeroPage,    bytes: 2, function: ora },
    Instruction { opcode: 0x06, name: "ASL", cycles: 5, addressing_mode: ZeroPage,    bytes: 2, function: asl },
    Instruction { opcode: 0x07, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x08, name: "PHP", cycles: 3, addressing_mode: Implicit,    bytes: 1, function: php },
    Instruction { opcode: 0x09, name: "ORA", cycles: 2, addressing_mode: Immediate,   bytes: 2, function: ora },
    Instruction { opcode: 0x0a, name: "ASL", cycles: 2, addressing_mode: Accumulator, bytes: 1, function: asl },
    Instruction { opcode: 0x0b, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x0c, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x0d, name: "ORA", cycles: 4, addressing_mode: Absolute,    bytes: 3, function: ora },
    Instruction { opcode: 0x0e, name: "ASL", cycles: 6, addressing_mode: Absolute,    bytes: 3, function: ora },
    Instruction { opcode: 0x0f, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    // 16
    Instruction { opcode: 0x10, name: "BPL", cycles: 2, addressing_mode: Relative,    bytes: 2, function: bpl },
    Instruction { opcode: 0x11, name: "ORA", cycles: 5, addressing_mode: IndirectY,   bytes: 2, function: ora },
    Instruction { opcode: 0x12, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x13, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x14, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x15, name: "ORA", cycles: 4, addressing_mode: ZeroPageX,   bytes: 2, function: ora },
    Instruction { opcode: 0x16, name: "ASL", cycles: 6, addressing_mode: ZeroPageX,   bytes: 2, function: asl },
    Instruction { opcode: 0x17, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x18, name: "CLC", cycles: 2, addressing_mode: Implicit,    bytes: 1, function: clc },
    Instruction { opcode: 0x19, name: "ORA", cycles: 4, addressing_mode: AbsoluteY,   bytes: 3, function: ora },
    Instruction { opcode: 0x1a, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x1b, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x1c, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x1d, name: "ORA", cycles: 4, addressing_mode: AbsoluteX,   bytes: 3, function: ora },
    Instruction { opcode: 0x1e, name: "ASL", cycles: 7, addressing_mode: AbsoluteX,   bytes: 3, function: asl },
    Instruction { opcode: 0x1f, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    // 32
    Instruction { opcode: 0x20, name: "JSR", cycles: 2, addressing_mode: Relative,    bytes: 2, function: jsr },
    Instruction { opcode: 0x21, name: "AND", cycles: 6, addressing_mode: IndirectY,   bytes: 2, function: and },
    Instruction { opcode: 0x22, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x23, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x24, name: "BIT", cycles: 3, addressing_mode: ZeroPage,    bytes: 2, function: bit },
    Instruction { opcode: 0x25, name: "AND", cycles: 3, addressing_mode: ZeroPage,    bytes: 2, function: and },
    Instruction { opcode: 0x26, name: "ROL", cycles: 5, addressing_mode: ZeroPage,    bytes: 2, function: rol },
    Instruction { opcode: 0x27, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x28, name: "PLP", cycles: 4, addressing_mode: Implicit,    bytes: 1, function: plp },
    Instruction { opcode: 0x29, name: "AND", cycles: 2, addressing_mode: Immediate,   bytes: 2, function: and },
    Instruction { opcode: 0x2a, name: "ROL", cycles: 2, addressing_mode: Accumulator, bytes: 2, function: rol },
    Instruction { opcode: 0x2b, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
    Instruction { opcode: 0x2c, name: "BIT", cycles: 4, addressing_mode: Absolute,    bytes: 3, function: bit },
    Instruction { opcode: 0x2d, name: "AND", cycles: 4, addressing_mode: Absolute,    bytes: 3, function: and },
    Instruction { opcode: 0x2e, name: "ROL", cycles: 6, addressing_mode: Absolute,    bytes: 3, function: rol },
    Instruction { opcode: 0x2f, name: "IVL", cycles: 0, addressing_mode: Implicit,    bytes: 0, function: invalid },
];


pub fn parse_instruction(opcode: u8) -> Instruction<'static> {
    OPTABLE[opcode as usize]
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