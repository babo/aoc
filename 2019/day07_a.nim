#!/usr/bin/env nim c -r
import strutils
import sequtils

import advent

proc shuffle(content: seq[int]): seq[seq[int]] =
  let n = content.len
  if n > 1:
    var rtv: seq[seq[int]]
    for h in 0 .. n - 1:
      for sab in shuffle(concat(content[0 .. h - 1], content[h + 1 .. n - 1])):
        rtv.add(concat(@[content[h]], sab))
    return rtv
  elif n == 1:
    return @[content]
  else:
    return @[]

proc calcThrust(phase: seq[int], code: seq[int]): int =
  var maxi = -1
  var signal = 0

  proc input(data: seq[int]): proc (): int =
    var data = data
    return proc (): int = return data.pop()

  for p in phase:
    discard intcode(code, 0, input(@[signal, p]), proc (x :int) = signal = x)
    if signal > maxi:
      maxi = signal
  return maxi

proc maxThrust(code: seq[int]): int =
  var maxi = 0
  for phase in shuffle((0 .. 4).toSeq()):
    let signal = calcThrust(phase, code)
    if signal > maxi:
      maxi = signal
  return maxi

assert calcThrust(@[4, 3, 2, 1, 0], @[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]) == 43210
assert maxThrust(@[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]) == 43210
assert maxThrust(@[3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]) == 54321
assert maxThrust(@[3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]) == 65210

let input = readInput("day07.data").split(',').map(parseInt)
echo maxThrust(input)