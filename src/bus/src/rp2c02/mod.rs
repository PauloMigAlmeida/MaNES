use crate::traits::{MainBusConnection, PPUBusConnection};

pub struct PPU {
    // C: tbl_name[2][1024]
    tbl_name: [[u8; 1024]; 2],
    tbl_palette: [u8; 32],
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            tbl_name: [[0; 1024]; 2],
            tbl_palette: [0; 32],
        }
    }

    pub fn clock(&mut self) {
        todo!("implement PPU clock")
    }

}

impl MainBusConnection for PPU {
    fn cpu_read_u8(&self, addr: u16, _read_only: bool) -> u8 {
        match addr {
            // Control
            0x0 => 0,
            // Mask
            0x1 => 0,
            // Status
            0x2 => 0,
            // OAM Address
            0x3 => 0,
            // OAM Data
            0x4 => 0,
            // Scroll
            0x5 => 0,
            // PPU Address
            0x6 => 0,
            // PPU Data
            0x7 => 0,
            _ => panic!("invalid address requested"),
        }
    }

    fn cpu_write_u8(&mut self, addr: u16, _value: u8) {
        match addr {
            // Control
            0x0 => {}
            // Mask
            0x1 => {}
            // Status
            0x2 => {}
            // OAM Address
            0x3 => {}
            // OAM Data
            0x4 => {}
            // Scroll
            0x5 => {}
            // PPU Address
            0x6 => {}
            // PPU Data
            0x7 => {}
            _ => panic!("invalid address on PPU"),
        };
    }
}

impl PPUBusConnection for PPU {
    fn ppu_write_u8(&mut self, _addr: u16, _value: u8) {
        unimplemented!()
    }

    fn ppu_read_u8(&mut self, _addr: u16, _read_only: bool) -> u8 {
        unimplemented!()
    }
}