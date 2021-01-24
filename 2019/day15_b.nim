#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils
import options
import tables
import random
import sets

type Robot = proc (a: proc (): int, b: proc (c: int): void)

discard initRand(12345)

proc findOxygen(runner: Robot): int =
  var backHome = false
  var wall = initTable[(int, int), int]()
  var explored = initTable[(int, int), int]()
  var goal: Option[(int, int)] = none((int, int))
  var knownGoal = none((int, int))
  var x = 0
  var y = 0
  var heading = 0

  var state = 0

  proc ifOK(direction: int = heading): (int, int) =
    var x = x
    var y = y
    case direction
    of 0: y += 1
    of 1: y -= 1
    of 2: x -= 1
    of 3: x += 1
    else: assert false
    return (x, y)

  proc input(): int =
    if goal.isSome():
      return 0

    let nowalls = (0..3).toSeq().filter(proc (h: int): bool = not wall.hasKey(ifOK(h)))
    let unexplored = nowalls.filter(proc (h: int): bool = not explored.hasKey(ifOK(h)))
    heading = if unexplored.len > 0: sample(unexplored) else: sample(nowalls)
    return heading + 1

  proc output(a: int): void =
    state = a
    let pos = ifOK()
    case state
      of 0:
        wall[pos] = heading
      of 1:
        if goal.isSome() and pos == (0, 0):
          backHome = true
        (x, y) = pos
        if not explored.hasKey(pos):
          explored[pos] = heading
      of 2:
        (x, y) = pos
        goal = some(pos)
        explored[pos] = heading
      else: assert false

  explored[(x, y)] = heading
  runner(input, output)
  for i in 0..10:
    x = 0
    y = 0
    heading = 1
    knownGoal = goal
    goal = none((int, int))
    runner(input, output)

  iterator neighbours(p: (int, int)): (int, int) =
    let (x, y) = p
    yield (x + 1, y)
    yield (x - 1, y)
    yield (x, y + 1)
    yield (x, y - 1)

  proc fillUp(): int =
    var count = -1
    var filled = initHashSet[(int, int)]()
    var cells = @[goal.get()]

    while cells.len > 0:
      count += 1
      echo count, " ", cells.len
      var next: seq[(int, int)] = @[]
      for c in cells:
        filled.incl(c)
        for n in neighbours(c):
          if wall.hasKey(n) or filled.contains(n):
            continue
          if explored.hasKey(n):
            next.add(n)
      cells = next
    return count

  return fillUp()

let input = readInput("day15.data").split(',').map(parseInt)
let score = findOxygen(proc (a: proc (): int, b: proc (c: int)): void = runIntCode(input, a, b))

echo score