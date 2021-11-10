// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct A {
   f: fn() -> &'static str
}
impl A {
   fn f(&self) -> &'static str {
       "The method f"
   }
}
let a = A{f: || "The field f"};

assert_eq!( a.f (), "The method f");
assert_eq!((a.f)(), "The field f");
}