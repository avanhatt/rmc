// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// rmc ~/rmc/src/test/cbmc/DynTrait/vtable_duplicate_fields.rs

trait A {
    fn foo(&self) -> i32;
}

trait B {
    fn foo(&self) -> i32;
}

trait T: A + B {}

struct Concrete {
    x: i32,
    y: i32,
}

impl Concrete {
    fn new(a: i32, b: i32) -> Concrete {
        Concrete { x: a, y: b }
    }
    fn new_box(a: i32, b: i32) -> Box<dyn T> {
        Box::new(Concrete::new(a, b))
    }
}

impl A for Concrete {
    fn foo(&self) -> i32 {
        self.x
    }
}

impl B for Concrete {
    fn foo(&self) -> i32 {
        self.y
    }
}

impl T for Concrete {}

fn main() {
    let t = Concrete::new_box(1, 2);
    let a = <dyn T as A>::foo(&*t);
    assert!(a == 1);
    let b = <dyn T as B>::foo(&*t);
    assert!(b == 2);

    println!("a: {}", a);
    println!("b: {}", b);
}
