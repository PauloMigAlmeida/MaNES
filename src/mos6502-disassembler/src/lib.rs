use bus::mos6502::*;

pub fn disassemble_program(machine_code: &[u8], base_address: u16, verbose: bool) -> String {
    if machine_code.is_empty() {
        panic!("program to be disassembled can't be empty");
    }

    let mut content = String::new();
    let mut i: u16 = 0;
    loop {
        let inst = OPTABLE[machine_code[i as usize] as usize];

        // handle invalid/unofficial opcodes
        if inst.name == "IVL" {
            if verbose {
                content.push_str(format!(
                    "{:04X}: {:02X} {:<8}",
                    base_address + i,
                    inst.opcode,
                    ""
                ).as_str());
            }
            content.push_str("??? \n");
            i += 1;
            continue;
        }

        // verbose mode adds absolute position + machine code (useful for GUI)
        if verbose {
            content.push_str(format!("{:04X}: {:02X} ", base_address + i, inst.opcode).as_str());

            if inst.bytes == 1 {
                content.push_str(format!("{:<8}", "").as_str());
            } else if inst.bytes == 2 {
                content
                    .push_str(format!("{:02X}{:<6}", machine_code[(i + 1) as usize], "").as_str());
            } else {
                content.push_str(
                    format!(
                        "{:02X} {:02X}{:<3}",
                        machine_code[(i + 1) as usize],
                        machine_code[(i + 2) as usize],
                        ""
                    )
                    .as_str(),
                );
            }
        }

        content.push_str(inst.name);
        content.push(' ');

        parse_arguments(&inst, &machine_code, &i, base_address, &mut content);

        i += inst.bytes as u16;
        if i as usize == machine_code.len() {
            break;
        }
    }

    content
}

fn parse_arguments(
    instruction: &Instruction,
    machine_code: &[u8],
    pos: &u16,
    base_address: u16,
    content: &mut String,
) {
    match instruction.mode {
        AddressingMode::Implicit => (),
        AddressingMode::Accumulator => {
            content.push_str("A");
        }
        AddressingMode::Immediate => {
            content.push_str(format!("#${:02X}", parse_u8(machine_code, pos)).as_str());
        }
        AddressingMode::ZeroPage => {
            content.push_str(format!("${:02X}", parse_u8(machine_code, pos)).as_str());
        }
        AddressingMode::ZeroPageX => {
            content.push_str(format!("${:02X},x", parse_u8(machine_code, pos)).as_str());
        }
        AddressingMode::ZeroPageY => {
            content.push_str(format!("${:02X},y", parse_u8(machine_code, pos)).as_str());
        }
        AddressingMode::Absolute => {
            content.push_str(format!("${:04X}", parse_u16(machine_code, pos)).as_str());
        }
        AddressingMode::AbsoluteX => {
            content.push_str(format!("${:04X},x", parse_u16(machine_code, pos)).as_str());
        }
        AddressingMode::AbsoluteY => {
            content.push_str(format!("${:04X},y", parse_u16(machine_code, pos)).as_str());
        }
        AddressingMode::Indirect => {
            content.push_str(format!("(${:04X})", parse_u16(machine_code, pos)).as_str());
        }
        AddressingMode::IndirectX => {
            content.push_str(format!("(${:02X},x)", parse_u8(machine_code, pos)).as_str());
        }
        AddressingMode::IndirectY => {
            content.push_str(format!("(${:02X},x)", parse_u8(machine_code, pos)).as_str());
        }
        AddressingMode::Relative => {
            // yeah yeah, I could do the bitwise dance but being able to emulate low-bit hardware
            // is that it allows you to be a bit sloppy without hurting performance too much...
            // It's a bit more readable this way..
            let mut rel_addr = base_address as i32;
            rel_addr += *pos as i32;
            rel_addr += instruction.bytes as i32;
            rel_addr += (parse_u8(machine_code, pos) as i8) as i32;

            let rel_addr = rel_addr as u16;
            content.push_str(format!("${:04X}", rel_addr).as_str());
        }
        _ => (),
    }
    content.push('\n');
}

fn parse_u16(machine_code: &[u8], pos: &u16) -> u16 {
    // 6502 is little endian
    machine_code[*pos as usize + 1] as u16 | (machine_code[*pos as usize + 2] as u16) << 8
}

fn parse_u8(machine_code: &[u8], pos: &u16) -> u8 {
    machine_code[*pos as usize + 1]
}

#[cfg(test)]
mod tests {
    use crate::disassemble_program;

    #[test]
    fn parse_simple_program() {
        // Address  Hexdump   Disassembly
        // -------------------------------
        // $0600    a9 01     LDA #$01
        // $0602    8d 00 02  STA $0200
        // $0605    a9 05     LDA #$05
        // $0607    8d 01 02  STA $0201
        // $060a    a9 08     LDA #$08
        // $060c    8d 02 02  STA $0202

        let machine_code: [u8; 16] = [
            0xca, 0xa9, 0x1, 0x8d, 0x0, 0x2, 0xa9, 0x5, 0x8d, 0x01, 0x2, 0xa9, 0x8, 0x8d, 0x02, 0x2,
        ];
        let content = disassemble_program(&machine_code, 0x0600, false);
        assert_eq!(content,"DEX \n\
                            LDA #$01\n\
                            STA $0200\n\
                            LDA #$05\n\
                            STA $0201\n\
                            LDA #$08\n\
                            STA $0202\n");
        let content = disassemble_program(&machine_code, 0x0600, true);
        assert_eq!(content,"0600: CA         DEX \n\
                            0601: A9 01      LDA #$01\n\
                            0603: 8D 00 02   STA $0200\n\
                            0606: A9 05      LDA #$05\n\
                            0608: 8D 01 02   STA $0201\n\
                            060B: A9 08      LDA #$08\n\
                            060D: 8D 02 02   STA $0202\n"
        );
    }

    #[test]
    fn parse_program_with_relative_modes() {
        // Address  Hexdump   Disassembly
        // -------------------------------
        // $0600    4c 05 06  JMP $0605
        // $0603    a2 15     LDX #$15
        // $0605    a2 02     LDX #$02
        // $0607    f0 07     BEQ $0610
        // $0609    ca        DEX
        // $060a    f0 04     BEQ $0610
        // $060c    ca        DEX
        // $060d    f0 f6     BEQ $0605
        // $060f    ca        DEX
        // $0610    a9 10     LDA #$10

        let machine_code: [u8; 18] = [
            0x4c, 0x5, 0x6, 0xa2, 0x15, 0xa2, 0x2, 0xf0, 0x7, 0xca, 0xf0, 0x4, 0xca, 0xf0, 0xf6,
            0xca, 0xa9, 0x10,
        ];
        let content = disassemble_program(&machine_code, 0x0600, false);
        assert_eq!(
            content,
            "JMP $0605\nLDX #$15\nLDX #$02\nBEQ $0610\nDEX \nBEQ $0610\nDEX \nBEQ $0605\nDEX \nLDA #$10\n"
        );
    }
}
