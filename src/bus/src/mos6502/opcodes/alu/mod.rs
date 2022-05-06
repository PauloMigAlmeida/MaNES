mod and;
mod ora;
mod eor;
mod cmp;
mod lda;
mod sta;
mod adc;
mod sbc;
pub use and::*;
pub use ora::*;
pub use eor::*;
pub use cmp::*;
pub use lda::*;
pub use sta::*;
pub use adc::*;
pub use sbc::*;

use crate::Bus;
use crate::mos6502::{Mos6502, Instruction, Flags::*};
#[allow(unused_imports)]
use crate::mos6502::{AddressingMode::*};
#[allow(unused_imports)]
use crate::mos6502::opcodes::{OPTABLE};