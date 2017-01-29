#!/bin/bash

set -e
set -x

cd rust_src
cargo build
cd ..

gcc -Wall example.c -Lrust_src/target/debug -lremacs -ldl -lpthread -o example
