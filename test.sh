#!/bin/bash
set -euo pipefail

cd test
RUST_BACKTRACE=1 cargo run input.son output.o
gcc -o output output.o
./output
