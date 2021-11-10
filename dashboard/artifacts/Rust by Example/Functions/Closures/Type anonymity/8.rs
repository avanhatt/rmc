// compile-flags: --edition 2018
#![allow(unused)]
// `F` must be generic.
pub fn main() {
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
}