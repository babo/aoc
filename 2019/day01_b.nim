#!/usr/bin/env nim c -r
import math
import streams
import strutils
import sequtils

proc fuelCalc(mass: int): int =
  var fuel = 0
  var step = mass
  while step > 0:
    step = max(int(step / 3) - 2, 0)
    fuel += step
  return fuel

const testVals = @[(12, 2), (14, 2), (1969, 966), (100756, 50346)]
assert all(testVals, proc (mf: (int, int)): bool = fuelCalc(mf[0]) == mf[1])

var strm = newFileStream("day01.data")
assert not isNil(strm)
let input = strm.readAll()
strm.close()

echo sum(splitLines(input).map(parseInt).map(fuelCalc))
