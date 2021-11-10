// compile-flags: --edition 2018
#![allow(unused)]
#![feature(box_patterns)]

pub fn main() {
    let b = Some(Box::new(5));
    match b {
        Some(box n) if n < 0 => {
            println!("Box contains negative number {}", n);
        },
        Some(box n) if n >= 0 => {
            println!("Box contains non-negative number {}", n);
        },
        None => {
            println!("No box");
        },
        _ => unreachable!()
    }
}