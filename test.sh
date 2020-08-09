#!/bin/bash
set -euo pipefail

RUST_BACKTRACE=1 cargo run
cd test
gcc -o output output.o
./output
