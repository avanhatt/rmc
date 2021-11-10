// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
loop {
    async move {
        break; // This would break out of the loop.
    }
}
}