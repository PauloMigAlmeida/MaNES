mod and;
mod ora;
mod eor;
mod cmp;
mod lda;
mod sta;
use bus::Bus;
use super::{Mos6502, Instruction};
pub use and::*;
pub use ora::*;
pub use eor::*;
pub use cmp::*;
pub use lda::*;
pub use sta::*;

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn adc(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn sbc(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}
