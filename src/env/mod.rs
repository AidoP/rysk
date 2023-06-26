use crate::Xlen;

mod hart;
pub use hart::Hart;

pub trait Addressable<X: Xlen> {
    fn read_u8(&self, address: X) -> u8;
    fn read_u16(&self, address: X) -> u16;
    fn read_u32(&self, address: X) -> u32;
    fn read_u64(&self, address: X) -> u64;

    fn write_u8(&self, address: X, byte: u8) -> Result<(), Cause<X>>;
    fn write_u16(&self, address: X, halfword: u16) -> Result<(), Cause<X>>;
    fn write_u32(&self, address: X, word: u32) -> Result<(), Cause<X>>;
    fn write_u64(&self, address: X, doubleword: u64) -> Result<(), Cause<X>>;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cause<X: Xlen>(X);
impl Cause<u32> {
    const INTERRUPT_BIT: u32 = 1 << (u32::BITS - 1);
    /// Returns `true` if the [`Cause`] is an interrupt.
    pub fn interrupt(self) -> bool {
        self.0 & Self::INTERRUPT_BIT != 0
    }

    pub const FETCH_MISALIGN: Self = Self(0);
    pub const FETCH_FAULT: Self = Self(1);
    pub const ILLEGAL_INSTRUCTION: Self = Self(2);
    pub const BREAKPOINT: Self = Self(3);
    pub const LOAD_MISALIGN: Self = Self(4);
    pub const LOAD_FAULT: Self = Self(5);
    pub const STORE_MISALIGN: Self = Self(6);
    pub const STORE_FAULT: Self = Self(7);
}
impl Cause<u64> {
    const INTERRUPT_BIT: u64 = 1 << (u64::BITS - 1);
    /// Returns `true` if the [`Cause`] is an interrupt.
    pub fn interrupt(self) -> bool {
        self.0 & Self::INTERRUPT_BIT != 0
    }

    pub const FETCH_MISALIGN: Self = Self(0);
    pub const FETCH_FAULT: Self = Self(1);
    pub const ILLEGAL_INSTRUCTION: Self = Self(2);
    pub const BREAKPOINT: Self = Self(3);
    pub const LOAD_MISALIGN: Self = Self(4);
    pub const LOAD_FAULT: Self = Self(5);
    pub const STORE_MISALIGN: Self = Self(6);
    pub const STORE_FAULT: Self = Self(7);
}
