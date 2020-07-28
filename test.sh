#!/bin/bash
set -euo pipefail

cargo run
cd test
gcc -o output test.c output.o
./output
