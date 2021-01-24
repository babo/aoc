#!/usr/bin/env nim c -r
import strutils

proc test(a: int): bool =
  if a < 100000 or a > 999999:
    return false
  let s = intToStr(a)
  for i in 0 .. s.len()-2:
    if ord(s[i+1]) < ord(s[i]):
      return false
  var j, i = 0
  while i < s.len() - 1:
    j = i + 1
    while s[i] == s[j] and j < s.len() - 1:
      j += 1
    if (s[i] != s[j] and i + 2 == j) or (s[i] == s[j] and i + 1 == j):
      return true
    i = j
  return false

assert test(111111) == false
assert test(223450) == false
assert test(123789) == false
assert test(111234) == false
assert test(123444) == false
assert test(112233) == true
assert test(123345) == true
assert test(112345) == true
assert test(111122) == true

echo toSeq(235741 .. 706948).map(test).filter(proc (x: bool): bool = x).len
