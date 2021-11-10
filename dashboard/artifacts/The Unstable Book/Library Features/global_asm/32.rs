// compile-flags: --edition 2018
#![allow(unused)]
#![feature(global_asm)]
pub fn main() {
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
mod x86 {

pub mod sally {
    global_asm!(
        ".global foo",
        "foo:",
        "jmp baz",
    );

    #[no_mangle]
    pub unsafe extern "C" fn baz() {}
}

// the symbols `foo` and `bar` are global, no matter where
// `global_asm!` was used.
extern "C" {
    fn foo();
    fn bar();
}

pub mod harry {
    global_asm!(
        ".global bar",
        "bar:",
        "jmp quux",
    );

    #[no_mangle]
    pub unsafe extern "C" fn quux() {}
}
}
}