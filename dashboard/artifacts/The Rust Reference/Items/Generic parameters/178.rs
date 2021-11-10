// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
// ok
pub fn main() {
struct Foo<const N: usize>;
enum Bar<const M: usize> { A, B }

// ERROR: unused parameter
struct Baz<T>;
struct Biz<'a>;
struct Unconstrained;
impl<const N: usize> Unconstrained {}
}