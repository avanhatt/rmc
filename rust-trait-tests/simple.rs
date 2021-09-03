trait T {
    fn f(&self);
}

struct S {}

impl T for S {
    fn f(&self) {}
}

fn main() {
    let d = &S{} as &dyn T;
    d.f();
}
