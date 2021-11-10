// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
let int_reference = &3;
match int_reference {
    &(0..=5) => (),
    _ => (),
}
}