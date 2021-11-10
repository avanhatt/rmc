// compile-flags: --edition 2018
#![allow(unused)]
// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

pub fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}