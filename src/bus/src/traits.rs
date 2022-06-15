pub trait MainBusConnection {
    fn cpu_read_u8(&self, addr: u16, read_only: bool) -> u8;

    fn cpu_write_u8(&mut self, addr: u16, value: u8);

    fn cpu_read_u16(&self, addr: u16, read_only: bool) -> u16 {
        let low = self.cpu_read_u8(addr, read_only);
        let high = self.cpu_read_u8(addr + 1, read_only);
        ((high as u16) << 8) | low as u16
    }

    fn cpu_write_u16(&mut self, addr: u16, value: u16) {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.cpu_write_u8(addr, low);
        self.cpu_write_u8(addr + 1, high);
    }
}

pub trait PPUBusConnection {
    fn ppu_write_u8(&mut self, _addr: u16, _value: u8);

    fn ppu_read_u8(&mut self, _addr: u16, _read_only: bool) -> u8;
}

