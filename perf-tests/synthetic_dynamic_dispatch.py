# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

import argparse
import subprocess

ITERATIONS = 10
FUNCTIONS = 2

trait_def = """
trait A {
    fn f(&self, x: i32) -> i32;
}
"""

struct_def = """
struct Struct{index} {{}}

impl A for Struct{index} {{
    fn f(&self, x: i32) -> i32 {{
        if x == {case} {{
            return -1;
        }}
        return 1;
    }}
}}
"""

main_def = """
fn main() {{
    for i in 0..{iterations} {{
        let s : &dyn A = if i % {functions} == 0 {{
            &Struct0 {{}} as &dyn A
        {body}
        }} else {{
            unreachable!()
        }};
        assert!(s.f(i) == 1); 
    }}
}}
"""

loop_body_case_def = """
        }} else if i % {functions} == {index} {{
            &Struct{index} {{}} as &dyn A
"""

smack_prelude = """
#[macro_use]
extern crate smack;
use smack::*;
use smack::assert;
"""

def build_rs(iterations, functions):
    rs = trait_def

    # Define a struct per dynamic function
    for i in range(functions):
        # Failure cases at the end of the iterations
        case = iterations - i
        if i == (iterations - 1) % functions:
            case = iterations - 1
        rs += struct_def.format(index = i, case = case)

    # Build the "if else" cases of the loop body
    body = ""
    for i in range(1, functions):
        body += loop_body_case_def.format(functions = functions, index = i)

    # Build the main body iteself
    rs += main_def.format(iterations = iterations, functions = functions, body = body)    

    return rs


def main():

    # Parse arguments
    parser = argparse.ArgumentParser(description='Run and time synthetic dynamic dispatch tests')
    parser.add_argument('--funs', type=int, help='Number of dynamic functions (minimum if maximum is provided)', default=2)
    parser.add_argument('--maxfuns', type=int, help='Maximum number of dynamic functions')
    parser.add_argument('--i', type=int, help='Number of iterations', default=10)
    parser.add_argument('--maxi', type=int, help='Maximum number of iterations (minimum if maximum is provided)')
    args = parser.parse_args()

    # Default to single parameters if no maximum provided
    maxfuns = args.maxfuns if args.maxfuns else args.funs
    maxi = args.maxi if args.maxi else args.i
    assert(maxfuns >= args.funs)
    assert(maxi >= args.i)

    for i in range(args.i, maxi + 1):
        for f in range(args.funs, maxfuns + 1):
            rs = build_rs(i, f)
            filename = "dynamic_dispatch_{}i_{}f.rs".format(i, f)
            print(filename)
            with open(filename, 'w') as writer:
                writer.write(rs)


if __name__ == "__main__":
    main()