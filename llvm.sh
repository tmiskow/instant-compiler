#!/usr/bin/env bash

cargo run > llvm/main.ll
llvm-as -o llvm/main.bc llvm/main.ll
llvm-link -o llvm/out.bc llvm/main.bc llvm/runtime.bc
lli llvm/out.bc
