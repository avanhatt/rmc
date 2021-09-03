// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Check drop implementation for a concrete, non-trait object.

#[macro_use]
extern crate smack;
use smack::*;

static mut CELL: i32 = 0;

struct Concrete1;

impl Drop for Concrete1 {
    fn drop(&mut self) {
        unsafe {
            CELL = 1;
        }
    }
}

fn main() {
    {
        let _x1 = Concrete1 {};
    }
    unsafe {
        smack::assert!(CELL == 1);
    }
}
