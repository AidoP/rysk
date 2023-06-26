use crate::{Register, Xlen};

use super::Addressable;

pub trait Hart<X: Xlen, A: Addressable<X>> {
    fn load_register(&self, r: Register) -> X;
    fn store_register(&mut self, r: Register, v: X);
    fn bus(&mut self) -> &mut A;
}