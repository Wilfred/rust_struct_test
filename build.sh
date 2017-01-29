#!/bin/bash

set -e
set -x

gcc -Wall example.c -Lrust_src/target/debug -lremacs -ldl -lpthread -o example
