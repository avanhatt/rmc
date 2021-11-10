// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn as_str(data: &u32) -> &str {
    let s = format!("{}", data);
    &s
}
}