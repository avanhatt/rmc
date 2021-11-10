// compile-flags: --edition 2018
#![allow(unused)]
mod foo {
    pub trait Zoo {
        fn zoo(&self) {}
    }

    impl<T> Zoo for T {}
}

use self::foo::Zoo as _;
struct Zoo;  // Underscore import avoids name conflict with this item.

pub fn main() {
    let z = Zoo;
    z.zoo();
}