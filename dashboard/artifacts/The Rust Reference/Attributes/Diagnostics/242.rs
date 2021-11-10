// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
#[must_use]
trait Critical {}
impl Critical for i32 {}

fn get_critical() -> impl Critical {
    4i32
}

// Violates the `unused_must_use` lint.
get_critical();
}