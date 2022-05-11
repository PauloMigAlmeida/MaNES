mod header;

use crate::cartridge::header::Header;
use std::fs::File;
use std::io::{BufReader, Read};

const PRG_ROM_SIZE_FACTOR:usize = 16384;
const CHR_ROM_SIZE_FACTOR:usize = 8192;

pub struct INESFormat {
    // Header (16 bytes)
    header: Header,
    // Trainer, if present (0 or 512 bytes)
    trainer: Vec<u8>,
    // PRG ROM data (16384 * x bytes)
    prg_rom: Vec<u8>,
    // CHR ROM data, if present (8192 * y bytes)
    chr_rom: Vec<u8>,
    // PlayChoice INST-ROM, if present (0 or 8192 bytes)
    plc_inst_rom: Vec<u8>,
    // PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing)
    plc_prom: Vec<u8>,
}

impl INESFormat {
    fn new() -> Self {
        INESFormat {
            header: Header::new(),
            trainer: vec![],
            prg_rom: vec![],
            chr_rom: vec![],
            plc_inst_rom: vec![],
            plc_prom: vec![],
        }
    }

    pub fn from(filename: &str) -> Self {
        let mut rom = INESFormat::new();
        let bytes = rom.read_file(filename).expect("err reading file");
        let mut pos = 0 as usize;

        rom.header = Header::from(&bytes).expect("invalid iNES Header");
        pos += 16;

        if rom.header.flags_6 & 0x4 == 0x4 {
            rom.trainer.copy_from_slice(&bytes[pos..(pos + 512)]);
            pos += 512;
        }

        rom.prg_rom.resize(rom.header.prg_rom_size as usize * PRG_ROM_SIZE_FACTOR, 0);
        rom.prg_rom.copy_from_slice(&bytes[pos..(pos + rom.header.prg_rom_size as usize * PRG_ROM_SIZE_FACTOR)]);
        pos += rom.prg_rom.len();

        rom.chr_rom.resize(rom.header.chr_rom_size as usize * CHR_ROM_SIZE_FACTOR, 0);
        rom.chr_rom.copy_from_slice(&bytes[pos..(pos + rom.header.chr_rom_size as usize * CHR_ROM_SIZE_FACTOR)]);

        rom
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
    use std::io::{Write};
    use std::os::unix::prelude::*;
    use tempfile::{NamedTempFile, tempfile};
    use filename::file_name;
    use super::*;

    fn generate_file(add_trainer: bool) -> (NamedTempFile, String) {
        let mut tmp_file = NamedTempFile::new().unwrap();

        // header
        let mut contents:Vec<u8> = vec![0x4E, 0x45, 0x53, 0x1A, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        if add_trainer {
            contents[6] |= 0x4;
            contents.resize(contents.len() + 512, 0xFF);
        }

        //  prg_rom
        contents.resize(contents.len() + contents[4] as usize * PRG_ROM_SIZE_FACTOR, 0xEE);

        //  chr_rom
        contents.resize(contents.len() + contents[5] as usize * CHR_ROM_SIZE_FACTOR, 0xDD);

        tmp_file.write_all(contents.as_slice()).expect("failed to write");
        tmp_file.flush();

        // filename
        let filename = file_name(&tmp_file.as_raw_fd()).unwrap();
        let os_str = filename.into_os_string();

        (tmp_file, String::from(os_str.to_str().unwrap()))
    }


    #[test]
    fn test_ines_parsing() {
        // No trainer
        let (tmp_file, filename) = generate_file( false);
        let rom = INESFormat::from(filename.as_str());

        assert_eq!(tmp_file.as_file().metadata().unwrap().len(), 24592);
        assert_eq!(rom.header.magic_const, [0x4E, 0x45, 0x53, 0x1A]);
        assert_eq!(rom.header.flags_6, 0);
        assert_eq!(rom.trainer.len(), 0);

        assert_eq!(rom.prg_rom.len(), 1 * PRG_ROM_SIZE_FACTOR);
        assert_eq!(&[0xEE as u8; 1 * PRG_ROM_SIZE_FACTOR], &rom.prg_rom[..]);

        assert_eq!(rom.chr_rom.len(), 1 * CHR_ROM_SIZE_FACTOR);
        assert_eq!(&[0xDD as u8; 1 * CHR_ROM_SIZE_FACTOR], &rom.chr_rom[..]);


        // With trainer
        let (tmp_file, filename) = generate_file( true);
        let rom = INESFormat::from(filename.as_str());

        assert_eq!(tmp_file.as_file().metadata().unwrap().len(), 25104);
        assert_eq!(rom.header.magic_const, [0x4E, 0x45, 0x53, 0x1A]);
        assert_eq!(rom.header.flags_6, 0);
        assert_eq!(rom.trainer.len(), 512);

        assert_eq!(rom.prg_rom.len(), 1 * PRG_ROM_SIZE_FACTOR);
        assert_eq!(&[0xEE as u8; 1 * PRG_ROM_SIZE_FACTOR], &rom.prg_rom[..]);

        assert_eq!(rom.chr_rom.len(), 1 * CHR_ROM_SIZE_FACTOR);
        assert_eq!(&[0xDD as u8; 1 * CHR_ROM_SIZE_FACTOR], &rom.chr_rom[..]);
    }

}
