// compile-flags: --edition 2018
#![allow(unused)]
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

pub fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}