// compile-flags: --edition 2018
#![allow(unused)]
fn foo() {}
fn bar() {
    self::foo();
}
pub fn main() {}