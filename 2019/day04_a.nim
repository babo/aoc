#!/usr/bin/env nim c -r
import math
import strutils
import sequtils

proc test(a: int): bool =
  if a < 100000 or a > 999999:
    return false
  let s = intToStr(a)
  var adj = 0
  for i in 0 .. s.len()-2:
    if ord(s[i+1]) < ord(s[i]):
      return false
    if s[i] == s[i+1]:
      adj += 1
  return adj > 0

assert test(111111) == true
assert test(223450) == false
assert test(123789) == false

echo toSeq(235741 .. 706948).map(test).filter(proc (x: bool): bool = x).len
