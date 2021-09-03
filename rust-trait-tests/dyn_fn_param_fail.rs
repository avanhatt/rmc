// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Check that we can pass a dyn function pointer to a stand alone
// function definition. Inverted negative test, expected to fail
// all asserts.

#![feature(ptr_metadata)]

macro_rules! vtable {
    ($f:ident) => {{
        unsafe {
            let ptr: *mut dyn std::any::Any = std::mem::transmute($f);
            std::ptr::metadata(ptr)
        }
    }};
}

fn size_from_vtable(vtable_ptr: std::ptr::DynMetadata<dyn std::any::Any>) -> usize {
    // 2nd usize-sized position
    vtable_ptr.size_of()
}

fn takes_dyn_fun(fun: &dyn Fn() -> u32) {
    let x = fun();
    assert!(x != 5);

    /* The function dynamic object has no associated data */
    assert!(size_from_vtable(vtable!(fun)) != 0);
}

pub fn unit_to_u32() -> u32 {
    assert!(false);
    5 as u32
}

fn main() {
    takes_dyn_fun(&unit_to_u32)
}
