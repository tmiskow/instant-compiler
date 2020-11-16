#!/usr/bin/env bash

for i in {1..7}; do
  filename="tests/test0$i.ins"
  ./insc_jvm $filename
  ./insc_llvm $filename
done

cd tests || exit

for i in {1..7}; do
  echo "$i:"
  if java "test0$i" |  diff - "test0$i.output"; then
    echo "JVM PASSED"
  else
    echo "JVM FAILED"
  fi
  if lli "test0$i.bc" |  diff - "test0$i.output"; then
    echo "LLVM PASSED"
  else
    echo "LLVM FAILED"
  fi
done
