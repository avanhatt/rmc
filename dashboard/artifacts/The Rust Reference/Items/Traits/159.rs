// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
// Not object safe if `Self` is a type argument.
pub fn main() {
trait Super<A> {}
trait WithSelf: Super<Self> where Self: Sized {}

struct S;
impl<A> Super<A> for S {}
impl WithSelf for S {}
let obj: Box<dyn WithSelf> = Box::new(S); // ERROR: cannot use `Self` type parameter
}