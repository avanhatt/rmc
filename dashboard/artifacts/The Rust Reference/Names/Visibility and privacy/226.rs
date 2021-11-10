// compile-flags: --edition 2018
#![allow(unused)]
pub use self::implementation::api;

mod implementation {
    pub mod api {
        pub fn f() {}
    }
}

pub fn main() {}