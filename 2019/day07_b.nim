#!/usr/bin/env nim c -r
import options
import sequtils
import strutils

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
  var signal = 0
  proc output(x: int): void =
    signal = x

  proc input(data: seq[int]): proc (): int =
    var data = data
    return proc (): int =
      let rtv = if data.len > 0: data.pop() else: signal
      return rtv

  var state = repeat(code, 5)
  var inst = repeat(0, 5)
  var i = 0
  var final = 0
  var inpData = @[0]
  while true:
    if inst[i] == 0:
      inpData = @[phase[i]]
    else:
      inpData = @[]
    let ret = intcode(state[i], inst[i], input(inpData), output)
    if ret.isSome():
      state[i] = ret.get()[0]
      inst[i] = ret.get()[1]
      if i == 4:
        final = signal
    elif i == 4:
      break
    i = (i + 1) %% 5

  return final

proc maxThrust(code: seq[int]): int =
  var maxi = 0
  for phase in shuffle((5 .. 9).toSeq()):
    let signal = calcThrust(phase, code)
    if signal > maxi:
      maxi = signal
  return maxi

assert calcThrust(@[9,8,7,6,5], @[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]) == 139629729
assert maxThrust(@[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]) == 139629729
assert maxThrust(@[3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]) == 18216

let input = readInput("day07.data").split(',').map(parseInt)
echo maxThrust(input)