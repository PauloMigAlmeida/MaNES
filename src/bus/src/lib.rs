/// Notes to myself:
///     - Implement some sort of subscribe mechanism that allow components to register their
///         ranges so we can iterate through the list to find out which of them read/write op is
///         meant to be carried out

const RAM_SIZE:u16 = 0xFFFF;

pub struct Bus {
    ram: [u8; RAM_SIZE as usize + 1]
}

impl Bus {
    pub fn new() -> Self {
        Bus{ ram: [0; RAM_SIZE as usize + 1] }
    }

    pub fn read_address(&self, addr: u16) -> u8 {
        //TODO implement logic to get data from the right component
        let value = self.ram[addr as usize];
        value
    }

    pub fn write_address(&mut self, addr: u16, value: u8) {
        //TODO implement logic to write data to the right component
        self.ram[addr as usize] = value;
    }
}