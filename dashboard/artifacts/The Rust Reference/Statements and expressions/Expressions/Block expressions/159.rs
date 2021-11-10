// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn is_unix_platform() -> bool {
    #[cfg(unix)] { true }
    #[cfg(not(unix))] { false }
}
}