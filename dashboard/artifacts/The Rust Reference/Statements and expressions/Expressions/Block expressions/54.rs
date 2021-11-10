// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct Struct;

impl Struct {
    fn consume_self(self) {}
    fn borrow_self(&self) {}
}

fn move_by_block_expression() {
    let s = Struct;

    // Move the value out of `s` in the block expression.
    (&{ s }).borrow_self();

    // Fails to execute because `s` is moved out of.
    s.consume_self();
}
}