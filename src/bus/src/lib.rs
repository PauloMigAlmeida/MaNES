use crate::cartridge::Cartridge;
use crate::mos6502::Mos6502;
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
    pub cpu: Mos6502,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_ram: [0; RAM_SIZE as usize + 1],
            system_clock: 0,
            cartridge: Cartridge::new(),
            ppu: PPU::new(),
            cpu: Mos6502::new(),
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
        {
            let mut new_cpu = self.cpu.clone();
            new_cpu.reset(self);
            self.cpu = new_cpu;
        }
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
        let mut data = 0x0_u8;
        if addr <= 0x1FFF {
            data = self.cpu_ram[(addr & 0x07FF) as usize];
        } else if (0x2000..=0x3FFF).contains(&addr) {
            data = self.ppu.cpu_read_u8(addr & 0x7, read_only);
        }
        data
        // panic!("invalid memory address requested... aborting")
    }

    fn cpu_write_u8(&mut self, addr: u16, value: u8) {
        if addr <= 0x1FFF {
            self.cpu_ram[(addr & 0x07FF) as usize] = value;
        } else if (0x2000..=0x3FFF).contains(&addr) {
            self.ppu.cpu_write_u8(addr & 0x7, value);
        }else {
            panic!("invalid memory address requested... aborting")
        }
    }
}

#[cfg(test)]
mod test;
