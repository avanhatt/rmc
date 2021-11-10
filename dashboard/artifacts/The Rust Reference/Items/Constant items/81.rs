// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
macro_rules! m {
    ($item: item) => { $item $item }
}

m!(const _: () = (););
// This expands to:
// const _: () = ();
// const _: () = ();
}