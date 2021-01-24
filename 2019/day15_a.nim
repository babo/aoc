#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils
import options
import tables
import random
import illwill

proc exitProc() {.noconv.} =
  illwillDeinit()
  showCursor()
  quit(0)

illwillInit(fullscreen=true)
setControlCHook(exitProc)
hideCursor()

# 2. We will construct the next frame to be displayed in this buffer and then
# just instruct the library to display its contents to the actual terminal
# (double buffering is enabled by default; only the differences from the
# previous frame will be actually printed to the terminal).
var tb = newTerminalBuffer(terminalWidth(), terminalHeight())
tb.setForegroundColor(fgGreen, true)
tb.display()

type Robot = proc (a: proc (): int, b: proc (c: int): void)

discard initRand(12345)

proc findOxygen(runner: Robot): Option[int] =
  var backHome = false
  var wall = initTable[(int, int), int]()
  var explored = initTable[(int, int), int]()
  var goal: Option[(int, int)] = none((int, int))
  var knownGoal = none((int, int))
  var x = 0
  var y = 0
  var heading = 0

  var state = 0

  proc draw(a: (int, int), t: string): void =
    tb.write(a[0] + 30, a[1] + 30, t)
    tb.display()

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
        draw(pos, "x")
      of 1:
        if goal.isSome() and pos == (0, 0):
          backHome = true
        (x, y) = pos
        if not explored.hasKey(pos):
          explored[pos] = heading
          draw(pos, ".")
      of 2:
        (x, y) = pos
        goal = some(pos)
        explored[pos] = heading
        draw(pos, "o")
      else: assert false

  explored[(x, y)] = heading
  draw((x, y), "S")
  runner(input, output)
  for i in 0..10:
    x = 0
    y = 0
    heading = 1
    knownGoal = goal
    goal = none((int, int))
    draw(knownGoal.get(), "o")
    draw((0, 0), "S")
    for k in wall.keys:
      draw(k, "x")
    runner(input, output)

  proc heuristic(a: (int, int)): int =
    assert goal.isSome()
    let dx = a[0] - goal.get()[0]
    let dy = a[1] - goal.get()[1]
    return dx * dx + dy * dy

  proc astar(): Option[int] =
    type Step = (int, int, int, Option[(int, int)])

    var openList = initTable[(int, int), Step]()
    var closedList = initTable[(int, int), Step]()

    var p = (0, 0)
    openList[p] = (heuristic(p), 0, heuristic(p), none((int, int)))
    while openlist.len > 0:
      var kmin: (int, int)
      var kval: Step
      var fmin = none(int)
      for k, v in openlist.pairs:
        if fmin.isNone() or v[0] < fmin.get():
          kmin = k
          kval = v
          fmin = some(v[0])

      closedList[kmin] = openlist[kmin]
      openlist.del(kmin)
      if kmin == goal.get():
        p = kmin
        break
      for dx in -1 .. 1:
        for dy in -1 .. 1:
          if dx * dy != 0 or (dx == 0 and dy == 0):
            continue
          let p = (kmin[0] + dx, kmin[1] + dy)
          if wall.hasKey(p) or closedList.hasKey(p):
            continue
          assert explored.hasKey(p), $p
          if openList.hasKey(p):
            var v = openList[p]
            if kval[1] < v[1]:
              v[1] = kval[1]
              v[0] = v[1] + v[2]
              v[3] = some(kmin)
          else:
            let g = kval[1] + 1
            let h = heuristic(p)
            openlist[p] = (g + h, g, h, some(kmin))

    if openlist.len == 0:
      return none(int)
    var count = 0
    p = goal.get()
    while closedList[p][3].isSome():
      count += 1
      p = closedList[p][3].get()
    return some(count)

  return astar()

let input = readInput("day15.data").split(',').map(parseInt)
let score = findOxygen(proc (a: proc (): int, b: proc (c: int)): void = runIntCode(input, a, b))

tb.clear()
tb.display()
echo score