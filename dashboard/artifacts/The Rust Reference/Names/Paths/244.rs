// compile-flags: --edition 2018
#![allow(unused)]
mod a {
    pub fn foo() {}
}
mod b {
    pub fn foo() {
        super::a::foo(); // call a's foo function
    }
}
pub fn main() {}