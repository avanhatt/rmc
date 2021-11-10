// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
pub enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}
}