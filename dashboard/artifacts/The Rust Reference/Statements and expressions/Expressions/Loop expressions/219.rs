// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
'outer: loop {
    while true {
        break 'outer;
    }
}
}