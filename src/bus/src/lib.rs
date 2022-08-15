use crate::cartridge::Cartridge;
use crate::rp2c02::PPU;
use crate::traits::MainBusConnection;

// Notes to myself:
//     - Implement some sort of subscribe mechanism that allow components to register their
//         ranges so we can iterate through the list to find out which of them read/write op is
//         meant to be carried out
//
//     - implement logic to write/read data to/from the right component in the bus
pub mod mos6502;
pub mod rp2c02;
pub mod inesformat;
pub mod cartridge;
pub mod traits;

const RAM_SIZE: u16 = 0x0800; // CPU has a whopping 2KB RAM
// const MAX_ROM_SIZE: usize = (RAM_SIZE - ROM_START_ADDR) as usize;
pub const ROM_START_ADDR: u16 = 0x8000;

pub struct Bus {
    cpu_ram: [u8; RAM_SIZE as usize + 1],
    system_clock: u64,
    cartridge: Cartridge,
    ppu: PPU,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_ram: [0; RAM_SIZE as usize + 1],
            system_clock: 0,
            cartridge: Cartridge::new(),
            ppu: PPU::new(),
        }
    }

    pub fn cpu_read_u8_slice(&self, from: u16, to: u16) -> &[u8] {
        if from <= 0x1FFF && to <= 0x1FFF && from < to {
            return &self.cpu_ram[((from & 0x07FF) as usize)..((to & 0x07FF) as usize)]
        }
        //TODO implement ppu cpu read if necessary (doesn't seem like it but we never know)
        panic!("invalid memory range requested... aborting")
    }

    pub fn load_cartridge(&mut self, filename: &str) -> Result<(), &str> {
        self.cartridge.load(filename).expect("failed to load cartridge");
        Ok(())
    }

    pub fn reset(&mut self) {
        self.cpu_ram =  [0; RAM_SIZE as usize + 1];
        self.cartridge.reset();
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

impl MainBusConnection for Bus {

    fn cpu_read_u8(&self, addr: u16, read_only: bool) -> u8 {
        let mut data = 0x0 as u8;
        if addr <= 0x1FFF {
            data = self.cpu_ram[(addr & 0x07FF) as usize];
        } else if addr >= 0x2000 && addr <= 0x3FFF {
            data = self.ppu.cpu_read_u8(addr & 0x7, read_only);
        }
        data
        // panic!("invalid memory address requested... aborting")
    }

    fn cpu_write_u8(&mut self, addr: u16, value: u8) {
        if addr <= 0x1FFF {
            self.cpu_ram[(addr & 0x07FF) as usize] = value;
        } else if addr >= 0x2000 && addr <= 0x3FFF {
            self.ppu.cpu_write_u8(addr & 0x7, value);
        }else {
            panic!("invalid memory address requested... aborting")
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use std::io::Write;
    use std::os::unix::prelude::*;
    use tempfile::NamedTempFile;
    use filename::file_name;
    use crate::inesformat::format::{CHR_ROM_SIZE_FACTOR, PRG_ROM_SIZE_FACTOR};

    pub fn generate_rom(add_trainer: bool, mapper_id: u8, ines_file_version: u8) -> (NamedTempFile, String) {
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

        if ines_file_version == 1 {
            //  prg_rom
            contents.resize(contents.len() + contents[4] as usize * PRG_ROM_SIZE_FACTOR, 0xEE);

            //  chr_rom
            contents.resize(contents.len() + contents[5] as usize * CHR_ROM_SIZE_FACTOR, 0xDD);
        } else if ines_file_version == 2 {
            contents[7] |= 0x08;
        }

        tmp_file.write_all(contents.as_slice()).expect("failed to write");

        // filename
        let filename = file_name(&tmp_file.as_raw_fd()).unwrap();
        let os_str = filename.into_os_string();

        (tmp_file, String::from(os_str.to_str().unwrap()))
    }

    #[test]
    fn test_memory_is_zeroed() {
        let bus = Bus::new();
        assert_eq!(&[0; RAM_SIZE as usize + 1], &bus.cpu_ram[..]);
    }

}
