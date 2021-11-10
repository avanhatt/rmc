// compile-flags: --edition 2018
#![allow(unused)]
static mut A: usize = 0;

pub fn main() {
    let t = std::thread::spawn(|| {
        unsafe { A += 1 };
    });
    unsafe { A += 1 };

    t.join().unwrap();
}