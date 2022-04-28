// Notes to myself:
//     - Implement some sort of subscribe mechanism that allow components to register their
//         ranges so we can iterate through the list to find out which of them read/write op is
//         meant to be carried out
//
//     - implement logic to write/read data to/from the right component in the bus

const RAM_SIZE: u16 = 0xFFFF;
const MAX_ROM_SIZE: usize = 0xFFFF - 0x8000;

pub struct Bus {
    ram: [u8; RAM_SIZE as usize + 1],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; RAM_SIZE as usize + 1],
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn read_u8_slice(&self, from: u16, to: u16) -> &[u8] {
        &self.ram[(from as usize)..(to as usize)]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read_u8(addr);
        let high = self.read_u8(addr + 1);
        ((high as u16) << 8) | low as u16
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.ram[addr as usize] = low;
        self.ram[(addr + 1) as usize] = high;
    }

    pub fn load_to_ram(&mut self, start: u16, content: &[u8]) {
        // sanity checks
        let rom_size: usize = if content.len() > MAX_ROM_SIZE { MAX_ROM_SIZE } else { content.len() };

        let mut j = start as usize;
        for i in 0..rom_size {
            self.ram[j] = content[i];
            j += 1;
        }
    }
}
