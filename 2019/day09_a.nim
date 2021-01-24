#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils
import algorithm

var repro = @[109'i64,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]

proc doNotCall(): int64 = assert false, "This should never be called"

proc eatIt(data: seq[int64]): proc (x: int64): void =
  var data: seq[int64] = reversed(data)
  return proc(x: int64): void =
    assert data.len > 0
    let should = data.pop()
    assert should == x

proc checkDigits(x: int64): void =
  assert x > high(int32)

runIntCode(repro, doNotCall, eatIt(repro))
runIntCode(@[104'i64, 1125899906842624, 99], doNotCall, eatIt(@[1125899906842624'i64]))

runIntCode(@[1102'i64,34915192,34915192,7,4,7,99,0], doNotCall, checkDigits)

let input = readInput("day09.data").split(',').map(parseBiggestInt)

runIntCode(input, proc (): int64 = 1, proc (x: int64) = echo x)