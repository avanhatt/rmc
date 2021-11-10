// compile-flags: --edition 2018
#![allow(unused)]
// Source
pub fn main() {
async fn example(x: &str) -> usize {
    x.len()
}
}