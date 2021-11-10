// compile-flags: --edition 2018
#![allow(unused)]
static mut P: *mut usize = std::ptr::null_mut();

pub fn main() {
    unsafe {
        {
            let mut x = 0;
            P = &mut x;
        }
        std::ptr::write_volatile(P, 123);
    }
}