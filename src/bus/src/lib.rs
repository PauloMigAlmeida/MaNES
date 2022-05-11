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

    #[test]
    fn test_memory_is_zeroed() {
        let bus = Bus::new();
        assert_eq!(&[0; RAM_SIZE as usize + 1], &bus.ram[..]);
    }

}