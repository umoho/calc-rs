#!/bin/bash

cargo build --release
archive=target/release/libcalc.a

gcc -Os calc.c $archive -o calc
