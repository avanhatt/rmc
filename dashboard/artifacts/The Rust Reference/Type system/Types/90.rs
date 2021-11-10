// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
use std::any::Any;
type T<'a> = &'a (dyn Any + Send);
}