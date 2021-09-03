// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// rmc-verify-fail

// Check that we can codegen a boxed dyn closure and fail an inner assertion

fn main() {
    // Create a boxed once-callable closure
    let f: Box<dyn FnOnce(i32)> = Box::new(|x| {
        assert!(x == 2);
    });

    // Call it
    f(1);
}
