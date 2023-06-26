#![no_std]

pub mod env;
mod instruction;

pub use env::{Addressable, Cause, Hart};
pub use instruction::{Instruction, Register};

pub trait Xlen {
    const BITS: usize;
    const MAX: usize;
}
impl Xlen for u32 {
    const BITS: usize = Self::BITS as _;
    const MAX: usize = Self::MAX as _;
}
impl Xlen for u64 {
    const BITS: usize = Self::BITS as _;
    const MAX: usize = Self::MAX as _;
}
