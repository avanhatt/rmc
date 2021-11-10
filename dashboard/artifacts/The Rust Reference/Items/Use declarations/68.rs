// compile-flags: --edition 2018
#![allow(unused)]
mod quux {
    pub use self::foo::{bar, baz};
    pub mod foo {
        pub fn bar() {}
        pub fn baz() {}
    }
}

pub fn main() {
    quux::bar();
    quux::baz();
}