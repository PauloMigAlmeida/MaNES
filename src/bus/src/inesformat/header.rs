pub struct Header {
    // 0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
    pub magic_const: [u8; 4],
    // 4: Size of PRG ROM in 16 KB units
    pub prg_rom_size: u8,
    // 5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
    pub chr_rom_size: u8,
    // 6: Flags 6 - Mapper, mirroring, battery, trainer
    //
    // 76543210
    // ||||||||
    // |||||||+- Mirroring: 0: horizontal (vertical arrangement) (CIRAM A10 = PPU A11)
    // |||||||              1: vertical (horizontal arrangement) (CIRAM A10 = PPU A10)
    // ||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
    // |||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
    // ||||+---- 1: Ignore mirroring control or above mirroring bit; instead provide four-screen VRAM
    // ++++----- Lower nybble of mapper number
    pub flags_6: u8,
    // 7: Flags 7 - Mapper, VS/Playchoice, NES 2.0
    //
    // 76543210
    // ||||||||
    // |||||||+- VS Unisystem
    // ||||||+-- PlayChoice-10 (8KB of Hint Screen data stored after CHR data)
    // ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
    // ++++----- Upper nybble of mapper number
    pub flags_7: u8,
    // 8: Flags 8 - PRG-RAM size (rarely used extension)
    pub flags_8: u8,
    // 9: Flags 9 - TV system (rarely used extension)
    pub flags_9: u8,
    // 10: Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
    //
    // 76543210
    //   ||  ||
    //   ||  ++- TV system (0: NTSC; 2: PAL; 1/3: dual compatible)
    //   |+----- PRG RAM ($6000-$7FFF) (0: present; 1: not present)
    //   +------ 0: Board has no bus conflicts; 1: Board has bus conflicts
    pub flags_10: u8,
    // 11-15: Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
    pub unused: [u8; 5],
}

pub enum HeaderVersion {
    V1,
    V2,
}

impl Header {
    pub fn new() -> Self {
        Header {
            magic_const: [0; 4],
            prg_rom_size: 0,
            chr_rom_size: 0,
            flags_6: 0,
            flags_7: 0,
            flags_8: 0,
            flags_9: 0,
            flags_10: 0,
            unused: [0; 5],
        }
    }

    pub fn from(content: &Vec<u8>) -> Result<Self, &str> {
        let mut ret = Header::new();

        let magic_const = &content[0..4];
        if magic_const != [0x4E, 0x45, 0x53, 0x1A] {
            return Err("Header Magic Constant doesn't match INES header format");
        }
        ret.magic_const.copy_from_slice(magic_const);

        let prg_rom_size = content[4];
        if prg_rom_size == 0 {
            return Err("prg_rom_size can't be 0");
        }
        ret.prg_rom_size = prg_rom_size;

        let chr_rom_size = content[5];
        if chr_rom_size == 0 {
            return Err("chr_rom_size can't be 0");
        }
        ret.chr_rom_size = chr_rom_size;
        ret.flags_6 = content[6];
        ret.flags_7 = content[7];
        ret.flags_8 = content[8];
        ret.flags_9 = content[9];
        ret.flags_10 = content[10];

        Ok(ret)
    }

    pub fn format_version(&self) -> HeaderVersion {
        if self.flags_7 & 0x0C == 0x08 {
            panic!("INES format V2 isn't support yet");
        }
        HeaderVersion::V1
    }

    pub fn mapper_id(&self) -> u8 {
        match self.format_version() {
            HeaderVersion::V1 => (self.flags_6 >> 4) | (self.flags_7 & 0xF0),
            _ => panic!("INES format V2 isn't support yet"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_validate_magic_const() {
        let x = vec![0x4E, 0x45, 0x53, 0x1A, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = Header::from(&x);
        assert!(result.is_ok());

        // wrong magic constant
        let x = vec![0x4E, 0x45, 0x53, 0x1B, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = Header::from(&x);
        assert!(result.is_err());
    }

    #[test]
    fn test_header_validate_min_sizes() {
        let x = vec![0x4E, 0x45, 0x53, 0x1A, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = Header::from(&x);
        assert!(result.is_ok());

        let x = vec![0x4E, 0x45, 0x53, 0x1A, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = Header::from(&x);
        assert!(result.is_err());

        let x = vec![0x4E, 0x45, 0x53, 0x1A, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = Header::from(&x);
        assert!(result.is_err());

        let x = vec![0x4E, 0x45, 0x53, 0x1A, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = Header::from(&x);
        assert!(result.is_err());
    }
}
