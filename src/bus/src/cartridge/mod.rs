use crate::inesformat::format::INESFormat;

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
        let rom = INESFormat::from(filename).expect("failed to parse rom");
        self.prg_rom.resize(rom.prg_rom.len(), 0);
        self.prg_rom.copy_from_slice(&rom.prg_rom[..]);
        self.chr_rom.resize(rom.chr_rom.len(), 0);
        self.chr_rom.copy_from_slice(&rom.chr_rom[..]);
        self.mapper_id = (rom.header.flags_6 >> 4) | (rom.header.flags_7 & 0xF0);
        Ok(())
    }

}
