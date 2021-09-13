# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

import argparse
import csv
from datetime import datetime
import os
import subprocess as sp
import time

ITERATIONS = 10
FUNCTIONS = 2
FIELDNAMES = ['tool', 'iterations', 'functions', 'time']

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
    """Build synthetic Rust file for given parameters"""
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

def run_rmc(results_dir, results_file, test, rust_file, iterations, functions):  
    """Run and time RMC on a given file"""
    rmc_cmd = ["rmc", rust_file]
    rmc_log_file = os.path.join(results_dir, "{}_rmc_log.txt".format(test))

    with open(rmc_log_file, 'w+') as log:
        start_time = time.time()
        gen = sp.Popen(rmc_cmd, stderr=log, stdout=log)
        gen.communicate()
        end_time = time.time()
        elapsed_time = end_time - start_time
        print("RMC for {} ran in {:.1f} seconds".format(test, elapsed_time))

    with open(rmc_log_file, 'r') as log:
        # Check for expected verification failure
        log_contents = log.read()
        if "VERIFICATION FAILED" not in log_contents:
            print("ERROR: expected verification failure not found for {}", test)
            exit(1)

    with open(results_file, 'a+') as results:
        writer = csv.DictWriter(results, fieldnames=FIELDNAMES)
        writer.writerow({
            'tool' : 'rmc', 
            'iterations' : iterations, 
            'functions' : functions, 
            'time' : elapsed_time,
        })


def run_smack(results_dir, results_file, test, rust_file, iterations, functions):  
    """Run and time SMACK on a given file"""
    smack_file = os.path.join(results_dir, "smack_{}.rs".format(test))
    with open(smack_file, 'w+') as smack:
        smack.write(smack_prelude)
        with open(rust_file, 'r') as rust:
            smack.write(rust.read())

    smack_cmd = ["smack", smack_file, "--unroll", "{}".format(iterations)]
    smack_log_file = os.path.join(results_dir, "{}_log.txt".format(test))

    with open(smack_log_file, 'w+') as log:
        start_time = time.time()
        gen = sp.Popen(smack_cmd, stderr=log, stdout=log)
        gen.communicate()
        end_time = time.time()
        elapsed_time = end_time - start_time
        print("SMACK for {} ran in {:.1f} seconds".format(test, elapsed_time))

    with open(smack_log_file, 'r') as log:
        # Check for expected verification failure
        log_contents = log.read()
        if "SMACK found an error" not in log_contents:
            print("ERROR: expected verification failure not found for {}", test)
            print(log_contents)
            exit(1)

    with open(results_file, 'a+') as results:
        writer = csv.DictWriter(results, fieldnames=FIELDNAMES)
        writer.writerow({
            'tool' : 'smack', 
            'iterations' : iterations, 
            'functions' : functions, 
            'time' : elapsed_time,
        })

def make_dir(d):
    """Makes a directory if it does not already exist"""
    if not os.path.exists(d):
        os.mkdir(d)

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

    # Create a subdirectory with the commit and datetime
    rev = sp.check_output(["git", "rev-parse", "--short", "HEAD"])
    rev = rev.decode("utf-8").strip()
    date = datetime.now().strftime('%Y-%m-%d_%H-%M')
    results_dir = 'synthetic_dynamic_dispatch_{}_{}'.format(date, rev)
    make_dir(results_dir)
    print("Writing results to: {}".format(results_dir))

    # Results CSV 
    results_file = os.path.join(results_dir, "results.csv")
    with open(results_file, 'w+') as results:
        writer = csv.DictWriter(results, fieldnames=FIELDNAMES)
        writer.writeheader()

    # Create each Rust file
    for i in range(args.i, maxi + 1, 10):
        for f in range(args.funs, maxfuns + 1):
            rs = build_rs(i, f)
            test = "{}i_{}f".format(i, f)
            rust_file = os.path.join(results_dir, "{}.rs".format(test))
            with open(rust_file, 'w') as rust:
                rust.write(rs)
            
            # Run RMC
            run_rmc(results_dir, results_file, test, rust_file, i, f)
            run_smack(results_dir, results_file, test, rust_file, i, f)

if __name__ == "__main__":
    main()