// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
type Surface = i32;
type BoundingBox = i32;
trait Shape {
    fn draw(&self, surface: Surface);
    fn bounding_box(&self) -> BoundingBox;
}
}