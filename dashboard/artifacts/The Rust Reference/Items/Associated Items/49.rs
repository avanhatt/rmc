// compile-flags: --edition 2018
#![allow(unused)]
struct Struct {
    field: i32
}

impl Struct {
    fn new() -> Struct {
        Struct {
            field: 0i32
        }
    }
}

pub fn main () {
    let _struct = Struct::new();
}