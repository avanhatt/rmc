// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
#![recursion_limit = "1"]

// This fails because it requires two recursive steps to auto-dereference.
pub fn main() {
(|_: &u8| {})(&&&1);
}