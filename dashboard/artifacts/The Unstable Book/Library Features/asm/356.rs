// compile-flags: --edition 2018
#![allow(unused)]
#![feature(asm, llvm_asm)]
pub fn main() {
fn load_fpu_control_word(control: u16) {
unsafe {
    asm!("fldcw [{}]", in(reg) &control, options(nostack));

    // Previously this would have been written with the deprecated `llvm_asm!` like this
    llvm_asm!("fldcw $0" :: "m" (control) :: "volatile");
}
}
}