// compile-flags: --edition 2018
#![allow(unused)]
fn foo() {}
mod a {
    fn bar() {
        crate::foo();
    }
}
pub fn main() {}