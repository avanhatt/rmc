// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
}