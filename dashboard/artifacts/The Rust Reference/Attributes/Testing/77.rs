// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
#[test]
#[should_panic(expected = "values don't match")]
fn mytest() {
    assert_eq!(1, 2, "values don't match");
}
}