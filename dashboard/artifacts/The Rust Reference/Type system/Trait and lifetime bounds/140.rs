// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}
}