mod stx;
mod nop;
mod ldx;
mod tsx;
mod txs;
mod txa;
mod tax;
mod inc;
mod dex;
mod dec;
mod asl;
mod lsr;
use bus::Bus;
use super::{Mos6502, Instruction};
pub use stx::*;
pub use nop::*;
pub use ldx::*;
pub use tsx::*;
pub use txs::*;
pub use txa::*;
pub use tax::*;
pub use inc::*;
pub use dex::*;
pub use dec::*;
pub use asl::*;
pub use lsr::*;

//TODO implement actual functions here... right now I'm just interested in the scaffold

pub fn rol(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

pub fn ror(cpu: &mut Mos6502, inst: Instruction, _bus: &mut Bus) -> u8 {
    println!("{} -> {:?} was called with cpu: {:?}", inst.name, inst.mode, cpu);
    0
}

