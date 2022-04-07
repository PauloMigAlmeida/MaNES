use bus::Bus;
use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn asl(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("asl was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn rol(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("rol was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn ror(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("ror was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn lsr(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("lsr was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}