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
    pub mode: AddressingMode,
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
    Invalid, // doesn't exist in 6502. I use that for illegal/unimplemented opcodes
}

const OPTABLE: [Instruction;256] = [
    Instruction { opcode: 0x00, name: "BRK", mode: Implicit,    bytes: 1, cycles: 7, function: brk },
    Instruction { opcode: 0x01, name: "ORA", mode: IndirectX,   bytes: 2, cycles: 6, function: ora },
    Instruction { opcode: 0x02, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x03, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x04, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x05, name: "ORA", mode: ZeroPage,    bytes: 2, cycles: 3, function: ora },
    Instruction { opcode: 0x06, name: "ASL", mode: ZeroPage,    bytes: 2, cycles: 5, function: asl },
    Instruction { opcode: 0x07, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x08, name: "PHP", mode: Implicit,    bytes: 1, cycles: 3, function: php },
    Instruction { opcode: 0x09, name: "ORA", mode: Immediate,   bytes: 2, cycles: 2, function: ora },
    Instruction { opcode: 0x0a, name: "ASL", mode: Accumulator, bytes: 1, cycles: 2, function: asl },
    Instruction { opcode: 0x0b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x0c, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x0d, name: "ORA", mode: Absolute,    bytes: 3, cycles: 4, function: ora },
    Instruction { opcode: 0x0e, name: "ASL", mode: Absolute,    bytes: 3, cycles: 6, function: ora },
    Instruction { opcode: 0x0f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x10, name: "BPL", mode: Relative,    bytes: 2, cycles: 2, function: bpl },
    Instruction { opcode: 0x11, name: "ORA", mode: IndirectY,   bytes: 2, cycles: 5, function: ora },
    Instruction { opcode: 0x12, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x13, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x14, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x15, name: "ORA", mode: ZeroPageX,   bytes: 2, cycles: 4, function: ora },
    Instruction { opcode: 0x16, name: "ASL", mode: ZeroPageX,   bytes: 2, cycles: 6, function: asl },
    Instruction { opcode: 0x17, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x18, name: "CLC", mode: Implicit,    bytes: 1, cycles: 2, function: clc },
    Instruction { opcode: 0x19, name: "ORA", mode: AbsoluteY,   bytes: 3, cycles: 4, function: ora },
    Instruction { opcode: 0x1a, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x1b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x1c, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x1d, name: "ORA", mode: AbsoluteX,   bytes: 3, cycles: 4, function: ora },
    Instruction { opcode: 0x1e, name: "ASL", mode: AbsoluteX,   bytes: 3, cycles: 7, function: asl },
    Instruction { opcode: 0x1f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x20, name: "JSR", mode: Relative,    bytes: 2, cycles: 2, function: jsr },
    Instruction { opcode: 0x21, name: "AND", mode: IndirectY,   bytes: 2, cycles: 6, function: and },
    Instruction { opcode: 0x22, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x23, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x24, name: "BIT", mode: ZeroPage,    bytes: 2, cycles: 3, function: bit },
    Instruction { opcode: 0x25, name: "AND", mode: ZeroPage,    bytes: 2, cycles: 3, function: and },
    Instruction { opcode: 0x26, name: "ROL", mode: ZeroPage,    bytes: 2, cycles: 5, function: rol },
    Instruction { opcode: 0x27, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x28, name: "PLP", mode: Implicit,    bytes: 1, cycles: 4, function: plp },
    Instruction { opcode: 0x29, name: "AND", mode: Immediate,   bytes: 2, cycles: 2, function: and },
    Instruction { opcode: 0x2a, name: "ROL", mode: Accumulator, bytes: 2, cycles: 2, function: rol },
    Instruction { opcode: 0x2b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x2c, name: "BIT", mode: Absolute,    bytes: 3, cycles: 4, function: bit },
    Instruction { opcode: 0x2d, name: "AND", mode: Absolute,    bytes: 3, cycles: 4, function: and },
    Instruction { opcode: 0x2e, name: "ROL", mode: Absolute,    bytes: 3, cycles: 6, function: rol },
    Instruction { opcode: 0x2f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x30, name: "BMI", mode: Relative,    bytes: 2, cycles: 2, function: bmi },
    Instruction { opcode: 0x31, name: "AND", mode: IndirectY,   bytes: 2, cycles: 5, function: and },
    Instruction { opcode: 0x32, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x33, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x34, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x35, name: "AND", mode: ZeroPageX,   bytes: 2, cycles: 4, function: and },
    Instruction { opcode: 0x36, name: "ROL", mode: ZeroPageX,   bytes: 2, cycles: 6, function: rol },
    Instruction { opcode: 0x37, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x38, name: "SEC", mode: Implicit,    bytes: 1, cycles: 1, function: sec },
    Instruction { opcode: 0x39, name: "AND", mode: AbsoluteY,   bytes: 3, cycles: 4, function: and },
    Instruction { opcode: 0x3a, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x3b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x3c, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x3d, name: "AND", mode: AbsoluteX,   bytes: 3, cycles: 4, function: and },
    Instruction { opcode: 0x3e, name: "ROL", mode: AbsoluteX,   bytes: 3, cycles: 7, function: rol },
    Instruction { opcode: 0x3f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x40, name: "rti", mode: Implicit,    bytes: 1, cycles: 6, function: rti },
    Instruction { opcode: 0x41, name: "ERO", mode: IndirectX,   bytes: 2, cycles: 6, function: eor },
    Instruction { opcode: 0x42, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x43, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x44, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x45, name: "EOR", mode: ZeroPage,    bytes: 2, cycles: 3, function: eor },
    Instruction { opcode: 0x46, name: "LSR", mode: ZeroPage,    bytes: 2, cycles: 5, function: lsr },
    Instruction { opcode: 0x47, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x48, name: "PHA", mode: Implicit,    bytes: 1, cycles: 3, function: pha },
    Instruction { opcode: 0x49, name: "EOR", mode: Immediate,   bytes: 2, cycles: 2, function: eor },
    Instruction { opcode: 0x4a, name: "LSR", mode: Accumulator, bytes: 1, cycles: 2, function: lsr },
    Instruction { opcode: 0x4b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x4c, name: "JMP", mode: Absolute,    bytes: 3, cycles: 3, function: jmp },
    Instruction { opcode: 0x4d, name: "EOR", mode: Absolute,    bytes: 3, cycles: 4, function: eor },
    Instruction { opcode: 0x4e, name: "LSR", mode: Absolute,    bytes: 3, cycles: 6, function: lsr },
    Instruction { opcode: 0x4f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x50, name: "BVC", mode: Relative,    bytes: 2, cycles: 2, function: bvc },
    Instruction { opcode: 0x51, name: "EOR", mode: IndirectY,   bytes: 2, cycles: 5, function: eor },
    Instruction { opcode: 0x52, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x53, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x54, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x55, name: "EOR", mode: ZeroPageX,   bytes: 2, cycles: 4, function: eor },
    Instruction { opcode: 0x56, name: "LSR", mode: ZeroPageX,   bytes: 2, cycles: 6, function: lsr },
    Instruction { opcode: 0x57, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x58, name: "CLI", mode: Implicit,    bytes: 1, cycles: 2, function: cli },
    Instruction { opcode: 0x59, name: "EOR", mode: AbsoluteY,   bytes: 3, cycles: 4, function: eor },
    Instruction { opcode: 0x5a, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x5b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x5c, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x5d, name: "EOR", mode: AbsoluteX,   bytes: 3, cycles: 4, function: eor },
    Instruction { opcode: 0x5e, name: "LSR", mode: AbsoluteX,   bytes: 3, cycles: 7, function: lsr },
    Instruction { opcode: 0x5f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x60, name: "RTS", mode: Implicit,    bytes: 1, cycles: 6, function: rts },
    Instruction { opcode: 0x61, name: "ADC", mode: IndirectX,   bytes: 2, cycles: 6, function: adc },
    Instruction { opcode: 0x62, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x63, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x64, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x65, name: "ADC", mode: ZeroPage,    bytes: 2, cycles: 3, function: adc },
    Instruction { opcode: 0x66, name: "ROR", mode: ZeroPage,    bytes: 2, cycles: 5, function: ror },
    Instruction { opcode: 0x67, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x68, name: "PLA", mode: Implicit,    bytes: 1, cycles: 4, function: pla },
    Instruction { opcode: 0x69, name: "ADC", mode: Immediate,   bytes: 2, cycles: 2, function: adc },
    Instruction { opcode: 0x6a, name: "ROR", mode: Accumulator, bytes: 2, cycles: 2, function: ror },
    Instruction { opcode: 0x6b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x6c, name: "JMP", mode: Indirect,    bytes: 3, cycles: 5, function: jmp },
    Instruction { opcode: 0x6d, name: "ADC", mode: Absolute,    bytes: 3, cycles: 4, function: adc },
    Instruction { opcode: 0x6e, name: "ROR", mode: Absolute,    bytes: 3, cycles: 6, function: ror },
    Instruction { opcode: 0x6e, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x70, name: "BVS", mode: Relative,    bytes: 2, cycles: 2, function: bvs },
    Instruction { opcode: 0x71, name: "ADC", mode: IndirectY,   bytes: 2, cycles: 5, function: adc },
    Instruction { opcode: 0x72, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x73, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x74, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x75, name: "ADC", mode: ZeroPageX,   bytes: 2, cycles: 4, function: adc },
    Instruction { opcode: 0x76, name: "ROR", mode: ZeroPageX,   bytes: 2, cycles: 6, function: ror },
    Instruction { opcode: 0x77, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x78, name: "SEI", mode: Implicit,    bytes: 1, cycles: 2, function: sei },
    Instruction { opcode: 0x79, name: "ADC", mode: AbsoluteY,   bytes: 3, cycles: 4, function: adc },
    Instruction { opcode: 0x7a, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x7b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x7c, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x7d, name: "ADC", mode: AbsoluteX,   bytes: 3, cycles: 4, function: adc },
    Instruction { opcode: 0x7e, name: "ROR", mode: AbsoluteX,   bytes: 3, cycles: 7, function: ror },
    Instruction { opcode: 0x7f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x80, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x81, name: "STA", mode: IndirectX,   bytes: 2, cycles: 6, function: sta },
    Instruction { opcode: 0x82, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x83, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x84, name: "STI", mode: ZeroPage,    bytes: 2, cycles: 3, function: sty },
    Instruction { opcode: 0x85, name: "STA", mode: ZeroPage,    bytes: 2, cycles: 3, function: sta },
    Instruction { opcode: 0x86, name: "STX", mode: ZeroPage,    bytes: 2, cycles: 3, function: stx },
    Instruction { opcode: 0x87, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x88, name: "DEY", mode: Implicit     bytes: 1, cycles: 2, function: dey },
    Instruction { opcode: 0x89, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x8a, name: "TXA", mode: Implicit     bytes: 1, cycles: 2, function: txa },
    Instruction { opcode: 0x8b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x8c, name: "STY", mode: Absolute     bytes: 3, cycles: 4, function: sty },
    Instruction { opcode: 0x8d, name: "STA", mode: Absolute     bytes: 3, cycles: 4, function: sta },
    Instruction { opcode: 0x8e, name: "STX", mode: Absolute     bytes: 3, cycles: 4, function: sta },
    Instruction { opcode: 0x8f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

    Instruction { opcode: 0x90, name: "BCC", mode: Relative,    bytes: 2, cycles: 2, function: bcc },
    Instruction { opcode: 0x91, name: "STA", mode: IndirectY,   bytes: 2, cycles: 6, function: sta },
    Instruction { opcode: 0x92, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x93, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x94, name: "STY", mode: ZeroPageX,   bytes: 2, cycles: 4, function: sty },
    Instruction { opcode: 0x95, name: "STA", mode: ZeroPageX,   bytes: 2, cycles: 4, function: sta },
    Instruction { opcode: 0x96, name: "STX", mode: ZeroPageY,   bytes: 2, cycles: 4, function: stx },
    Instruction { opcode: 0x97, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x98, name: "TYA", mode: Implicit,    bytes: 1, cycles: 2, function: tya },
    Instruction { opcode: 0x99, name: "STA", mode: AbsoluteY,   bytes: 3, cycles: 5, function: sta },
    Instruction { opcode: 0x9a, name: "TXS", mode: Implicit,    bytes: 1, cycles: 2, function: txs },
    Instruction { opcode: 0x9b, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x9c, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x9d, name: "STA", mode: AbsoluteX,   bytes: 3, cycles: 5, function: sta },
    Instruction { opcode: 0x9e, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },
    Instruction { opcode: 0x9f, name: "IVL", mode: Invalid,     bytes: 0, cycles: 0, function: invalid },

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