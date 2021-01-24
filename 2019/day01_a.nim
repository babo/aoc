#!/usr/bin/env nim c -r
import math
import streams
import strutils
import sequtils

proc fuelCalc(mass: int): int = int(mass / 3) - 2

const testVals = @[(12, 2), (14, 2), (1969, 654), (100756, 33583)]
assert all(testVals, proc (mf: (int, int)): bool = fuelCalc(mf[0]) == mf[1])

var strm = newFileStream("day01.data")
assert not isNil(strm)
let input = strm.readAll()
strm.close()

echo sum(splitLines(input).map(parseInt).map(fuelCalc))
