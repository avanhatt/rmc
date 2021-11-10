// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}
}