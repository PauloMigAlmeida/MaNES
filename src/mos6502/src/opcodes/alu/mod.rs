mod and;
mod ora;
mod eor;
mod cmp;
mod lda;
mod sta;
mod adc;
mod sbc;
use super::{Mos6502, Instruction};
pub use and::*;
pub use ora::*;
pub use eor::*;
pub use cmp::*;
pub use lda::*;
pub use sta::*;
pub use adc::*;
pub use sbc::*;

