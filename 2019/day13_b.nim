#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils
import sugar
import illwill

type Gamer = proc (a: proc (): int, b: proc (c: int): void)

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

proc arcade(runner: Gamer): int =
  var ball = (0, 0)
  var paddle = (0, 0)
  var x = 0
  var y = 0
  var state = 0
  var score = 0

  proc input(): int =
    return system.cmp(ball[0], paddle[0])

  proc output(a: int): void =
    case state
      of 0: x = a
      of 1: y = a
      of 2:
        if x == -1 and y == 0:
          score = a
          tb.write(0, 30, $score)
        else:
          case a
            of 0: tb.write(x, y, " ")
            of 1: tb.write(x, y, ".")
            of 2:
              tb.write(x, y, "X")
            of 3:
              paddle = (x, y)
              tb.write(x, y, "_")
            of 4:
              ball = (x, y)
              tb.write(x, y, "o")
            else: assert false
        tb.display()
      else: assert false

    state = (state + 1) mod 3

  runner(input, output)
  return score

var input = readInput("day13.data").split(',').map(parseInt)
input[0] = 2
let score = arcade((a: proc (): int, b: proc (c: int)) => runIntCode(input, a, b))

tb.clear()
tb.display()
echo score