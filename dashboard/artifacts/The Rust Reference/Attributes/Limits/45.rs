// rmc-codegen-fail
// compile-flags: --edition 2018
#![allow(unused)]
#![type_length_limit = "4"]

pub fn main() {
fn f<T>(x: T) {}

// This fails to compile because monomorphizing to
// `f::<((((i32,), i32), i32), i32)>` requires more than 4 type elements.
f(((((1,), 2), 3), 4));
}