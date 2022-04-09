use bus::Bus;
use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn ora(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("ora was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn adc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("adc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn and(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("and was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn eor(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("eor was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn sta(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("sta was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn lda(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("lda was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn cmp(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("cmp was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}

pub fn sbc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &Bus) {
    println!("sbc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
}