use crate::inesformat::format::INESFormat;
use std::mem::swap;

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper_id: u8,
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            prg_rom: vec![],
            chr_rom: vec![],
            mapper_id: 0,
        }
    }

    // everything leads me to believe that I might have to save more data into the cartridge
    // structure but right now I can't think of anything else I need... so future Paulo, take
    // a look at that.
    pub fn load(&mut self, filename: &str) -> Result<(), &str> {
        let mut rom = INESFormat::from(filename).expect("failed to parse rom");
        swap(&mut self.prg_rom, &mut rom.prg_rom);
        swap(&mut self.chr_rom, &mut rom.chr_rom);
        self.mapper_id = (rom.header.flags_6 >> 4) | (rom.header.flags_7 & 0xF0);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::inesformat::format::{CHR_ROM_SIZE_FACTOR, PRG_ROM_SIZE_FACTOR};
    use crate::test::generate_rom;

    #[test]
    fn test_load_cartridge() {
        let (tmp_file, filename) = generate_rom(false, 0);
        let mut cartridge = Cartridge::new();
        cartridge.load(filename.as_str()).expect("Failed loading file");

        assert_eq!(cartridge.prg_rom.len(), 1 * PRG_ROM_SIZE_FACTOR);
        assert_eq!(cartridge.chr_rom.len(), 1 * CHR_ROM_SIZE_FACTOR);
        assert_eq!(cartridge.mapper_id, 0);
        assert_eq!(&[0xEE as u8; 1 * PRG_ROM_SIZE_FACTOR], &cartridge.prg_rom[..]);
        assert_eq!(&[0xDD as u8; 1 * CHR_ROM_SIZE_FACTOR], &cartridge.chr_rom[..]);
    }

    #[test]
    fn test_mapper_id_value_retrieval() {
        let (tmp_file, filename) = generate_rom(false, 1);
        let mut cartridge = Cartridge::new();
        cartridge.load(filename.as_str()).expect("Failed loading file");

        assert_eq!(cartridge.mapper_id, 1);

        // test mappers which contains two nibbles
        let (tmp_file, filename) = generate_rom(false, 0xfe);
        let mut cartridge = Cartridge::new();
        cartridge.load(filename.as_str()).expect("Failed loading file");

        assert_eq!(cartridge.mapper_id, 0xfe);
    }
}
