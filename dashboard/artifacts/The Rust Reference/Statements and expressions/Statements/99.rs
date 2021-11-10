// compile-flags: --edition 2018
#![allow(unused)]
// bad: the block's type is i32, not ()
// Error: expected `()` because of default return type
// if true {
//   1
// }

// good: the block's type is i32
pub fn main() {
if true {
  1
} else {
  2
};
}