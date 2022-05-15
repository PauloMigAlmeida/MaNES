use crate::cartridge::Cartridge;

// Notes to myself:
//     - Implement some sort of subscribe mechanism that allow components to register their
//         ranges so we can iterate through the list to find out which of them read/write op is
//         meant to be carried out
//
//     - implement logic to write/read data to/from the right component in the bus
pub mod mos6502;
pub mod inesformat;
pub mod cartridge;

const RAM_SIZE: u16 = 0x0800; // CPU has a whopping 2KB RAM
// const MAX_ROM_SIZE: usize = (RAM_SIZE - ROM_START_ADDR) as usize;
pub const ROM_START_ADDR: u16 = 0x8000;

pub struct Bus {
    ram: [u8; RAM_SIZE as usize + 1],
    system_clock: u64,
    cartridge: Cartridge,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; RAM_SIZE as usize + 1],
            system_clock: 0,
            cartridge: Cartridge::new(),
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            return self.ram[(addr & 0x07FF) as usize]
        }
        panic!("invalid memory address requested... aborting")
    }

    pub fn read_u8_slice(&self, from: u16, to: u16) -> &[u8] {
        if from <= 0x1FFF && to <= 0x1FFF && from < to {
            return &self.ram[((from & 0x07FF) as usize)..((to & 0x07FF) as usize)]
        }
        panic!("invalid memory range requested... aborting")
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read_u8(addr);
        let high = self.read_u8(addr + 1);
        ((high as u16) << 8) | low as u16
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        if addr <= 0x1FFF {
            self.ram[(addr & 0x07FF) as usize] = value;
        } else {
            panic!("invalid memory address requested... aborting")
        }
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.write_u8(addr, low);
        self.write_u8(addr + 1, high);
    }

    pub fn load_cartridge(&mut self, filename: &str) -> Result<(), &str> {
        self.cartridge.load(filename).expect("failed to load cartridge");
        Ok(())
    }

    pub fn reset(&mut self) {
        //cpu.reset()
        self.system_clock = 0;
    }

    pub fn clock(&mut self) {
        //cpu.reset()
        self.system_clock += 1;
        if self.system_clock % 3 == 0 {
            //cpu.clock()
        }
        //ppu.clock()
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use std::io::{Write};
    use std::os::unix::prelude::*;
    use tempfile::NamedTempFile;
    use filename::file_name;
    use crate::inesformat::format::{CHR_ROM_SIZE_FACTOR, PRG_ROM_SIZE_FACTOR};

    pub fn generate_rom(add_trainer: bool, mapper_id: u8) -> (NamedTempFile, String) {
        let mut tmp_file = NamedTempFile::new().unwrap();

        // header
        let mut contents:Vec<u8> = vec![
            0x4E,
            0x45,
            0x53,
            0x1A,
            1,
            1,
            (mapper_id & 0x0F) << 4, // flag6
            (mapper_id & 0xF0),      // flag7
            0, 0, 0, 0, 0, 0, 0, 0];

        if add_trainer {
            contents[6] |= 0x4;
            contents.resize(contents.len() + 512, 0xFF);
        }

        //  prg_rom
        contents.resize(contents.len() + contents[4] as usize * PRG_ROM_SIZE_FACTOR, 0xEE);

        //  chr_rom
        contents.resize(contents.len() + contents[5] as usize * CHR_ROM_SIZE_FACTOR, 0xDD);

        tmp_file.write_all(contents.as_slice()).expect("failed to write");

        // filename
        let filename = file_name(&tmp_file.as_raw_fd()).unwrap();
        let os_str = filename.into_os_string();

        (tmp_file, String::from(os_str.to_str().unwrap()))
    }

    #[test]
    fn test_memory_is_zeroed() {
        let bus = Bus::new();
        assert_eq!(&[0; RAM_SIZE as usize + 1], &bus.ram[..]);
    }

}