// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
(0..10).collect::<Vec<_>>();
Vec::<u8>::with_capacity(1024);
}