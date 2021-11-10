// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
enum Link {
    Next(Box<Link>),
    None,
}
}