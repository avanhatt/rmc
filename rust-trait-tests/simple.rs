#[macro_use]
extern crate smack;
use smack::*;

trait T {
    fn f(&self) -> i32;
}

struct S {}

impl T for S {
    fn f(&self) -> i32 { 1 }
}

fn main() {
    let d = &S{} as &dyn T;
    smack::assert!(d.f() == 2);
}
