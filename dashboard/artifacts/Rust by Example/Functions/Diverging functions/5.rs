// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn foo() -> ! {
    panic!("This call never returns.");
}
}