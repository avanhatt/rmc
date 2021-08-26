// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

trait A {
    fn f(&self, x: i32) -> i32;
}

struct Struct1 {}

impl A for Struct1 {
    fn f(&self, x: i32) -> i32 {
        if x == 9999 {
            return -1;
        }
        return 1;
    }
}

struct Struct2 {}

impl A for Struct2 {
    fn f(&self, x: i32) -> i32 {
        if x == 9998 {
            return -1;
        }
        return 1;
    }
}

fn main() {
    for i in 0..10000 {
        if i % 2 == 1 {
            let s1 = Struct1 {};
            assert!(s1.f(i) == 1);
        } else {
            let s2 = Struct2 {};
            assert!(s2.f(i) == 1);
        }
    }
}
