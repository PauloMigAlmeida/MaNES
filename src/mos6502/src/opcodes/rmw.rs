use bus::Bus;
use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn asl(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("asl was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn rol(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("rol was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn ror(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("ror was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn lsr(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("lsr was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn stx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("stx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn txa(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("tax was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn txs(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("txs was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn ldx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("ldx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn tax(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("tax was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn tsx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("tsx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn dec(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("dec was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn dex(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("dex was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn inc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("dex was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn nop(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("nop was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}