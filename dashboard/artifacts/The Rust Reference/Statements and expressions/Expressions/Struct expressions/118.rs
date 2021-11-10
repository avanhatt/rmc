// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct Gamma;
let a = Gamma;  // Gamma unit value.
let b = Gamma{};  // Exact same value as `a`.
}