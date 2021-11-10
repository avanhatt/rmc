// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct List<T> {
  data: T,
  next: Option<Box<List<T>>>,
}
}