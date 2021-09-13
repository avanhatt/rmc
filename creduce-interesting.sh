#!/bin/bash
rustc test.rs >/dev/null 2>&1 &&\
rmc test.rs | grep 'BinaryOperation Expression does not typecheck Equal' >/dev/null 2>&1