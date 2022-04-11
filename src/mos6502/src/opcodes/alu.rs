use bus::Bus;
use super::{Mos6502, AddressingMode};

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn ora(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("ora was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn adc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("adc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn and(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("and was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn eor(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("eor was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn sta(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("sta was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn lda(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("lda was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn cmp(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("cmp was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}

pub fn sbc(cpu: &mut Mos6502, addr_mode: AddressingMode, bus: &mut Bus) -> u8 {
    println!("sbc was called with cpu: {:?} and addr_mode: {:?}", cpu, addr_mode);
    0
}