#!/usr/bin/env bash

cargo run > llvm/main.j
java -jar jasmin.jar llvm/main.j > /dev/null
java Main
