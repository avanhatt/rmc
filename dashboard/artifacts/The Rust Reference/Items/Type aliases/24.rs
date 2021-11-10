// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct MyStruct(u32);

use MyStruct as UseAlias;
type TypeAlias = MyStruct;

let _ = UseAlias(5); // OK
let _ = TypeAlias(5); // Doesn't work
}