mod header;

use crate::cartridge::header::Header;
use std::fs::File;
use std::io::{BufReader, Read};

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

        rom.prg_rom
            .copy_from_slice(&bytes[pos..(pos + rom.header.prg_rom_size as usize * 16384)]);
        pos += rom.prg_rom.len();

        rom.chr_rom
            .copy_from_slice(&bytes[pos..(pos + rom.header.chr_rom_size as usize * 8192)]);

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
