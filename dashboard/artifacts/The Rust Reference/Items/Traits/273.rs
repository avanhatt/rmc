// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
trait T {
    fn f1((a, b): (i32, i32)) {}
    fn f2(_: (i32, i32));  // Cannot use tuple pattern without a body.
}
}