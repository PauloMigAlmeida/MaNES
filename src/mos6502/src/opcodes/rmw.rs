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

pub fn stx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("stx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn txa(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("tax was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn txs(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("txs was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn ldx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("ldx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn tax(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("tax was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn tsx(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("tsx was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn dec(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("dec was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn dex(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("dex was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn inc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("dex was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn nop(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("nop was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}