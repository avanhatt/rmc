// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn foo() {
    // m!(); // Error: m is not in scope.
    macro_rules! m {
        () => {};
    }
    m!();
}


// m!(); // Error: m is not in scope.
}