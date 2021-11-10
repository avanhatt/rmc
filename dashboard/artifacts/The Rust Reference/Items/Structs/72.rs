// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct Cookie {}
const Cookie: Cookie = Cookie {};
let c = [Cookie, Cookie {}, Cookie, Cookie {}];
}