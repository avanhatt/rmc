// compile-flags: --edition 2018
#![allow(unused)]
// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

pub fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}