// compile-flags: --edition 2018
#![allow(unused)]
// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
pub fn main() {
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);
}