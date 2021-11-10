// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct Person {
   name: String,
   age: u8,
}
let person = Person{ name: String::from("John"), age: 23 };
// `name` is moved from person and `age` referenced
let Person { name, ref age } = person;
}