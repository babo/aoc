#!/usr/bin/env nim c -r
import streams
import strutils
import sequtils

proc lineToData(x: string): seq[int] = split(x, ',').map(parseInt)

proc intcode(input: seq[int]): seq[int] =
  var code = input
  var instruction = 0
  while true:
    assert instruction >= 0 and instruction < code.len
    case code[instruction]
      of 1: code[code[instruction + 3]] = code[code[instruction + 1]] + code[code[instruction + 2]]
      of 2: code[code[instruction + 3]] = code[code[instruction + 1]] * code[code[instruction + 2]]
      of 99: break
      else: echo "error in the code"
    instruction += 4
  return code

const testData = """
1,0,0,0,99|2,0,0,0,99
2,3,0,3,99|2,3,0,6,99
2,4,4,5,99,0|2,4,4,5,99,9801
1,1,1,4,99,5,6,0,99|30,1,1,4,2,5,6,0,99"""

for line in testData.splitLines():
  let parts = line.split('|').map(lineToData)
  assert intcode(parts[0]) == parts[1]

var strm = newFileStream("day02.data")
assert not isNil(strm)
let input = strm.readAll()
strm.close()

var data = lineToData(input)
for a in (0..99):
  for b in (0..99):
    data[1] = a
    data[2] = b
    let result = intcode(data)
    if result[0] == 19690720:
      echo a*100 + b
      break
