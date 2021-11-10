// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
pub mod inner {
    #[macro_export]
    macro_rules! call_foo {
        () => { $crate::inner::foo() };
    }

    pub fn foo() {}
}
}