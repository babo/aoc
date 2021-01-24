#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils

type Gamer = proc (a: proc (): int, b: proc (c: int): void)

proc arcade(runner: Gamer): int =
  var count = 0
  var state = 0

  proc input(): int =
    assert false
    return 0

  proc output(a: int): void =
    if state == 2 and a == 2:
      count += 1
    state = (state + 1) mod 3

  runner(input, output)

  return count

let input = readInput("day13.data").split(',').map(parseInt)
echo arcade(proc (a: proc (): int, b: proc (c: int)): void = runIntCode(input, a, b))
