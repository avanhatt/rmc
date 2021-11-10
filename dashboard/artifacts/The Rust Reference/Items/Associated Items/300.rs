// compile-flags: --edition 2018
#![allow(unused)]
trait ConstantId {
    const ID: i32;
}

struct Struct;

impl ConstantId for Struct {
    const ID: i32 = 1;
}

pub fn main() {
    assert_eq!(1, Struct::ID);
}