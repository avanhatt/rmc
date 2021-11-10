// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
}