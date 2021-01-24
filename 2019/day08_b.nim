#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils

let input = readInput("day08.data")

var final = repeat('2', 150)

for layer in distribute(input.toSeq(), input.len /% 150):
  for i in 0 .. 149:
    if final[i] == '2':
      final[i] = layer[i]

for line in distribute(final.replace('0', ' ').replace('1', '@').toSeq(), 6):
  echo join(line, "")