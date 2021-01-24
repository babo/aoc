#!/usr/bin/env nim c -r
import streams
import strutils
import sequtils

proc lineToData(x: string): seq[int] = split(x, ',').map(parseInt)

proc getInput: int = 1

proc putOutput(x: int): void = echo x

proc intcode(input: seq[int]): seq[int] =
  var code = input
  var instruction = 0
  var opcode = 0

  proc mode(inst: int, arg: int): int =
    var d: int = 0
    case arg
      of 1: d = 100
      of 2: d = 1000
      of 3: d = 10000
      else: assert arg > 0 and arg < 4
    return (code[inst] %% (10 * d)) /% d

  proc get(inst: int, arg: int): int =
    let m = mode(inst, arg)
    case m
      of 0: return code[code[inst + arg]]
      of 1: return code[inst + arg]
      else: assert m >= 0 and m < 2

  proc put(inst: int, arg: int, value: int): void =
    let m = mode(inst, arg)
    case m
      of 0: code[code[inst + arg]] = value
      of 1: code[inst + arg] = value
      else: assert m >= 0 and m < 2

  while true:
    assert instruction >= 0 and instruction < code.len
    case code[instruction] %% 100
      of 1: put(instruction, 3, get(instruction, 1) + get(instruction, 2)); instruction += 4
      of 2: put(instruction, 3, get(instruction, 1) * get(instruction, 2)); instruction += 4
      of 3: put(instruction, 1, getInput()); instruction += 2
      of 4: putOutput(get(instruction, 1)); instruction += 2
      of 99: instruction += 1; break
      else: echo "error in the code: ", code[instruction], instruction; doAssert(false, "szia")

  return code

const testData = """
1,0,0,0,99|2,0,0,0,99
2,3,0,3,99|2,3,0,6,99
2,4,4,5,99,0|2,4,4,5,99,9801
1,1,1,4,99,5,6,0,99|30,1,1,4,2,5,6,0,99"""

for line in testData.splitLines():
  let parts = line.split('|').map(lineToData)
  assert intcode(parts[0]) == parts[1]

var strm = newFileStream("day05.data")
assert not isNil(strm)
let input = strm.readAll()
strm.close()

var data = lineToData(input)
let result = intcode(data)
