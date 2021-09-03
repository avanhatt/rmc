// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Check drop implementation for an &dyn dynamic trait object.

#[macro_use]
extern crate smack;
use smack::*;

static mut CELL: i32 = 0;

trait T {
    fn t(&self) {}
}

struct Concrete1;

impl T for Concrete1 {}

impl Drop for Concrete1 {
    fn drop(&mut self) {
        unsafe {
            CELL = 1;
        }
    }
}

struct Concrete2;

impl T for Concrete2 {}

impl Drop for Concrete2 {
    fn drop(&mut self) {
        unsafe {
            CELL = 2;
        }
    }
}

fn main() {
    {
        let _x1: &dyn T = &Concrete1 {};
    }
    unsafe {
        smack::assert!(CELL == 1);
    }
    {
        let _x2: &dyn T = &Concrete2 {};
    }
    unsafe {
        smack::assert!(CELL == 2);
    }
}
