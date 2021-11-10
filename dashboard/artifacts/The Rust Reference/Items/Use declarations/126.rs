#![allow(unused)]
mod foo {
    pub mod example { pub mod iter {} }
    pub mod baz { pub fn foobaz() {} }
}
use foo::example::iter;
use ::foo::baz::foobaz;
pub fn main() {}