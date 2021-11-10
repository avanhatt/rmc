// compile-flags: --edition 2018
#![allow(unused)]
struct Foo {}

trait Bar {
  fn bar(&self);
}

impl Foo {
  fn bar(&mut self) {
    println!("In struct impl!")
  }
}

impl Bar for Foo {
  fn bar(&self) {
    println!("In trait impl!")
  }
}

pub fn main() {
  let mut f = Foo{};
  f.bar();
}