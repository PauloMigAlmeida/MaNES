use std::fs::File;
use std::io::{BufReader, Read};

// Ref: https://www.nesdev.org/wiki/INES
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
}

struct Header {
    // 0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
    magic_const: [u8; 4],
    // 4: Size of PRG ROM in 16 KB units
    prg_rom_size: u8,
    // 5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
    chr_rom_size: u8,
    // 6: Flags 6 - Mapper, mirroring, battery, trainer
    flags_6: u8,
    // 7: Flags 7 - Mapper, VS/Playchoice, NES 2.0
    flags_7: u8,
    // 8: Flags 8 - PRG-RAM size (rarely used extension)
    flags_8: u8,
    // 9: Flags 9 - TV system (rarely used extension)
    flags_9: u8,
    // 10: Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
    flags_10: u8,
    // 11-15: Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
    unused: [u8; 5],
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
}

fn read_file(filename: &str) -> Result<Vec<u8>, &str> {
    let file = File::open(filename).expect("file doesn't exist");
    let mut buf = BufReader::new(file);
    let mut bytes = Vec::new();
    buf.read_to_end(&mut bytes)
        .expect("failed to read the file");
    Ok(bytes)
}

fn read_header(content: &Vec<u8>) -> Result<Header, &str> {
    let mut ret = Header::new();

    let magic_const = &content[0..3];
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

pub fn read_cartridge_rom(filename: &str) -> Result<INESFormat, &str> {
    let bytes = read_file(filename)?;
    let mut nes_rom = INESFormat::new();
    nes_rom.header = read_header(&bytes).expect("invalid iNES Header");

    Ok(nes_rom)
}
