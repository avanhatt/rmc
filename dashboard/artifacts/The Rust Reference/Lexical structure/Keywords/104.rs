// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
// error[E0262]: invalid lifetime parameter name: `'static`
pub fn main() {
fn invalid_lifetime_parameter<'static>(s: &'static str) -> &'static str { s }
}