#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils

proc strToIntSeq(input: string): seq[int] =
  let one = input.toSeq().map(proc (n: char): int = ord(n) - ord('0'))
  return cycle(one, 10000)

iterator patternp(n: int, maxi: int): int =
  assert n >= 0
  var i: int = n
  var c = true
  while c and i < maxi:
    for j in 0 .. n:
      if i + j >= maxi:
        c = false
        break
      yield i + j
    i += 4 * n + 4

iterator patternm(n: int, maxi: int): int =
  assert n >= 0
  var i: int = n + 2 * n + 2
  var c = true
  while c and i < maxi:
    for j in 0 .. n:
      if i + j >= maxi:
        c = false
        break
      yield i + j
    i += 4 * n + 4

proc fft(input: seq[int], phases: int, offset: int = 0): seq[int] =
  let m = input.len
  var a = input
  var x: int
  for ph in 1 .. phases:
    for i in 0 .. m - 1:
      x = 0
      for p in patternp(i, m):
        x += a[p]
      for p in patternm(i, m):
        x -= a[p]
      assert x > low(int)
      assert x < high(int)
      a[i] = toU32(abs(x) mod 10)
    echo "Done: ", ph

  return a[offset .. offset + 7]

proc decode(raw: string): string =
  let input = strToIntSeq(raw)
  assert input.len < high(int)
  let offset = parseInt(raw[0 .. 6])
  let ri = fft(input, 100, offset)
  return join(ri, "")

assert fft(@[1, 2, 3, 4, 5, 6, 7, 8], 4) == @[0,1,0,2,9,4,9,8]

assert decode("03036732577212944063491565474664") == "84462026"
echo "OK 1"
assert decode("02935109699940807407585447034323") == "78725270"
echo "OK 2"
assert decode("03081770884921959731165446850517") == "53553731"
echo "OK 3"

let input = readInput("day16.data")
let result = decode(input)

echo result
