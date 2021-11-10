// compile-flags: --edition 2018
#![allow(unused)]
fn some_fn() {
    ()
}

pub fn main() {
    let a: () = some_fn();
    println!("This function returns and you can see this line.")
}