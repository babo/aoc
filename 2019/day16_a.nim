#!/usr/bin/env nim c -r
import advent
import sequtils

proc strToIntSeq(input: string): seq[int] = input.toSeq().map(proc (n: char): int = ord(n) - ord('0'))

iterator patterni(base: seq[int], n: int, m: int): int =
  assert n > 0
  var j: int = 0
  var c = true
  while c:
    for x in base:
      for i in 1 .. n:
        if j != 0:
          yield x
        j += 1
        if j > m:
          c = false
          break
      if not c:
        break

proc pattern(base: seq[int], n: int, m: int): seq[int] =
  var rtv: seq[int]
  for x in patterni(base, n, m):
    rtv.add(x)
  return rtv

proc fft(input: seq[int], phases: int, offset: int = 0): seq[int] =
  let base: seq[int] = @[0, 1, 0, -1]
  let m = input.len
  var input = input
  var x: int64
  var j: int
  for ph in 1 .. phases:
    for i in 1 .. m:
      x = 0
      j = 0
      for p in patterni(base, i, m):
        case p
          of 1: x += input[j]
          of -1: x -= input[j]
          else: discard
        j += 1
      input[i - 1] = toU32(abs(x) mod 10)

  return input[offset .. offset + 7]

assert pattern(@[1, 2, 3], 1, 3) == @[2, 3, 1]
assert pattern(@[1, 2, 3], 1, 6) == @[2, 3, 1, 2, 3, 1]
assert pattern(@[1, 2, 3], 2, 3) == @[1, 2, 2]
assert pattern(@[1, 2, 3], 2, 3) == @[1, 2, 2]
assert pattern(@[1, 2, 3], 2, 11) == @[1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3]
assert pattern(@[1, 2, 3], 3, 3) == @[1, 1, 2]

assert fft(strToIntSeq("12345678"), 1) == strToIntSeq("48226158")
assert fft(strToIntSeq("12345678"), 2) == strToIntSeq("34040438")
assert fft(strToIntSeq("12345678"), 3) == strToIntSeq("03415518")
assert fft(strToIntSeq("12345678"), 4) == strToIntSeq("01029498")

let input = strToIntSeq(readInput("day16.data"))
let result = fft(input, 100)

echo result
