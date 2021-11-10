// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
use std::io::{self, Write};
mod m {
    #[clippy::cyclomatic_complexity = "0"]
    pub (in super) fn f1() {}
}
}