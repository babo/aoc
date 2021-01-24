#!/usr/bin/env nim c -r
import advent
import sequtils

let input = readInput("day08.data")
echo input.len
echo input.len /% (25 * 6)

var maxi = [-1, 0, 0]
var now = [0, 0, 0]

for layer in distribute(input.toSeq(), input.len /% (25 * 6)):
  now = [0, 0, 0]
  for x in layer:
    now[ord(x) - ord('0')] += 1

  if maxi[0] == -1 or now[0] < maxi[0]:
    maxi = now

echo maxi[1] * maxi[2]
