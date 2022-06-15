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
mod rol;
mod ror;
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
pub use rol::*;
pub use ror::*;

use crate::Bus;
use crate::mos6502::{Flags::*, Instruction, Mos6502};
#[allow(unused_imports)]
use crate::mos6502::{AddressingMode::*};
#[cfg(test)]
use crate::mos6502::opcodes::OPTABLE;
use crate::traits::MainBusConnection;