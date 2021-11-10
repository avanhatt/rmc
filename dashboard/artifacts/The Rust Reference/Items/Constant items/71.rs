// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
const _: () =  { struct _SameNameTwice; };

// OK although it is the same name as above:
const _: () =  { struct _SameNameTwice; };
}