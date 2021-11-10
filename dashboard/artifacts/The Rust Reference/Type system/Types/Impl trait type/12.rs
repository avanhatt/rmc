// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
trait Trait {}
impl Trait for () {}

// argument position: anonymous type parameter
fn foo(arg: impl Trait) {
}

// return position: abstract return type
fn bar() -> impl Trait {
}
}