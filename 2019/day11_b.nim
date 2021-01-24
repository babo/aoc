#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils
import tables
import sugar

type Painting = Table[(int, int), int]

type Painter = proc (a: proc (): int64, b: proc (c: int64): void)

proc paintRobot(runner: Painter): Painting =
  var painted: Painting
  var x = 0
  var y = 0
  var heading = 0
  var state = 0

  proc input(): int64 =
    return painted.getOrDefault((x, y), 1)

  proc output(a: int64): void =
    if state == 0:
      painted[(x, y)] = if a == 0'i64: 0 else: 1
    else:
      let turn = if a == 0'i64: 3 else: 1
      heading = (heading + turn) mod 4
      case heading
        of 0: y -= 1
        of 1: x += 1
        of 2: y += 1
        of 3: x -= 1
        else: assert false, "Invalid heading"
    state = (state + 1) mod 2

  runner(input, output)

  return painted

proc display(painting: Painting): void =
  var ax = 0
  var ay = 0
  var bx = 0
  var by = 0

  for k in painting.keys:
    ax = min(ax, k[0])
    bx = max(bx, k[0])
    ay = min(ay, k[1])
    by = max(by, k[1])

  echo ax, " ", bx
  echo ay, " ", by

  for y in ay .. by:
    var line: seq[char]
    for x in ax .. bx:
      case painting.getOrDefault((x, y), 2)
        of 0: line.add(' ')
        of 1: line.add('x')
        of 2: line.add(' ')
        else: assert false
    echo join(line, "")

let input = readInput("day11.data").split(',').map(parseBiggestInt)
let painting = paintRobot((a: proc (): int64, b: proc (c: int64)) => runIntCode(input, a, b))
display(painting)