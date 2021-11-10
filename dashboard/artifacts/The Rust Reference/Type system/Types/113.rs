// compile-flags: --edition 2018
// rmc-flags: --cbmc-args --unwind 0
#![allow(unused)]
pub fn main() {
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

let a: List<i32> = List::Cons(7, Box::new(List::Cons(13, Box::new(List::Nil))));
}