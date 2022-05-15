use super::*;
use super::header::*;

pub const PRG_ROM_SIZE_FACTOR:usize = 16384;
pub const CHR_ROM_SIZE_FACTOR:usize = 8192;

pub struct INESFormat {
    // Header (16 bytes)
    pub header: Header,
    // Trainer, if present (0 or 512 bytes)
    pub trainer: Vec<u8>,
    // PRG ROM data (16384 * x bytes)
    pub prg_rom: Vec<u8>,
    // CHR ROM data, if present (8192 * y bytes)
    pub chr_rom: Vec<u8>,
    // PlayChoice INST-ROM, if present (0 or 8192 bytes)
    pub plc_inst_rom: Vec<u8>,
    // PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing)
    pub plc_prom: Vec<u8>,
}

impl INESFormat {
    pub fn new() -> Self {
        INESFormat {
            header: Header::new(),
            trainer: vec![],
            prg_rom: vec![],
            chr_rom: vec![],
            plc_inst_rom: vec![],
            plc_prom: vec![],
        }
    }

    pub fn from(filename: &str) -> Result<Self,&str> {
        let mut rom = INESFormat::new();
        let bytes = rom.read_file(filename).expect("err reading file");
        let mut pos = 0 as usize;

        rom.header = Header::from(&bytes).expect("invalid iNES Header");
        pos += 16;

        if rom.header.flags_6 & 0x4 == 0x4 {
            rom.trainer.resize(512,0);
            rom.trainer.copy_from_slice(&bytes[pos..(pos + 512)]);
            pos += 512;
        }

        rom.prg_rom.resize(rom.header.prg_rom_size as usize * PRG_ROM_SIZE_FACTOR, 0);
        rom.prg_rom.copy_from_slice(&bytes[pos..(pos + rom.header.prg_rom_size as usize * PRG_ROM_SIZE_FACTOR)]);
        pos += rom.prg_rom.len();

        rom.chr_rom.resize(rom.header.chr_rom_size as usize * CHR_ROM_SIZE_FACTOR, 0);
        rom.chr_rom.copy_from_slice(&bytes[pos..(pos + rom.header.chr_rom_size as usize * CHR_ROM_SIZE_FACTOR)]);

        Ok(rom)
    }

    fn read_file(&self, filename: &str) -> Result<Vec<u8>, &str> {
        let file = File::open(filename).expect("file doesn't exist");
        let mut buf = BufReader::new(file);
        let mut bytes = Vec::new();
        buf.read_to_end(&mut bytes)
            .expect("failed to read the file");
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::generate_rom;

    #[test]
    fn test_ines_parsing() {
        // No trainer
        let (tmp_file, filename) = generate_rom(false, 0);
        let rom = INESFormat::from(filename.as_str()).unwrap();

        assert_eq!(tmp_file.as_file().metadata().unwrap().len(), 24592);
        assert_eq!(rom.header.magic_const, [0x4E, 0x45, 0x53, 0x1A]);
        assert_eq!(rom.header.flags_6, 0);
        assert_eq!(rom.trainer.len(), 0);
        assert_eq!(rom.plc_inst_rom.len(), 0);
        assert_eq!(rom.plc_prom.len(), 0);

        assert_eq!(rom.prg_rom.len(), 1 * PRG_ROM_SIZE_FACTOR);
        assert_eq!(&[0xEE as u8; 1 * PRG_ROM_SIZE_FACTOR], &rom.prg_rom[..]);

        assert_eq!(rom.chr_rom.len(), 1 * CHR_ROM_SIZE_FACTOR);
        assert_eq!(&[0xDD as u8; 1 * CHR_ROM_SIZE_FACTOR], &rom.chr_rom[..]);

        // With trainer
        let (tmp_file, filename) = generate_rom(true, 0);
        let rom = INESFormat::from(filename.as_str()).unwrap();

        assert_eq!(tmp_file.as_file().metadata().unwrap().len(), 25104);
        assert_eq!(rom.header.magic_const, [0x4E, 0x45, 0x53, 0x1A]);
        assert_eq!(rom.header.flags_6, 0x4);
        assert_eq!(rom.trainer.len(), 512);
        assert_eq!(rom.plc_inst_rom.len(), 0);
        assert_eq!(rom.plc_prom.len(), 0);

        assert_eq!(rom.prg_rom.len(), 1 * PRG_ROM_SIZE_FACTOR);
        assert_eq!(&[0xEE as u8; 1 * PRG_ROM_SIZE_FACTOR], &rom.prg_rom[..]);

        assert_eq!(rom.chr_rom.len(), 1 * CHR_ROM_SIZE_FACTOR);
        assert_eq!(&[0xDD as u8; 1 * CHR_ROM_SIZE_FACTOR], &rom.chr_rom[..]);
    }

}
