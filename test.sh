#!/usr/bin/env bash

#for i in {1..6}; do
#  echo "$i:"
#  filename="tests/test0$i"
#  ./llvm.sh < "$filename.ins" 2>/dev/null |  diff "$filename.output" -
#done


for i in {1..6}; do
  echo "$i:"
  filename="tests/test0$i"
  ./jvm.sh < "$filename.ins" 2>/dev/null |  diff "$filename.output" -
done
