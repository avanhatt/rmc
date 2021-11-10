// compile-flags: --edition 2018
#![allow(unused)]
#![feature(arbitrary_self_types, generator_trait)]
pub fn main() {
use std::ops::GeneratorState;
use std::pin::Pin;

pub trait Generator<R = ()> {
    type Yield;
    type Return;
    fn resume(self: Pin<&mut Self>, resume: R) -> GeneratorState<Self::Yield, Self::Return>;
}
}