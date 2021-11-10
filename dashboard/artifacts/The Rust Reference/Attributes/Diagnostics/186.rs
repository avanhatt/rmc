// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
#[deprecated(since = "5.2", note = "foo was rarely used. Users should instead use bar")]
pub fn foo() {}

pub fn bar() {}
}