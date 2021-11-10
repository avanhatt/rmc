// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn outer() {
  let outer_var = true;

  fn inner() { /* outer_var is not in scope here */ }

  inner();
}
}