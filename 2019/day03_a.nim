#!/usr/bin/env nim c -r
import streams
import strutils
import tables

iterator spitCoords(line: string): (int, int) =
  var x = 0
  var y = 0

  for word in line.split(','):
    let d = parseInt(substr(word, 1))
    for i in 1..d:
      case word[0]
        of 'U': y += 1
        of 'D': y -= 1
        of 'L': x += 1
        of 'R': x -= 1
        else: echo "YAY"
      yield (x, y)

proc mh(p: (int, int)): int = abs(p[0]) + abs(p[1])

proc closest(a: string, b: string): int =
  var dots = initTable[(int, int), int]()

  for c in spitCoords(a):
    dots[c] = mh(c)

  var found = 0
  for c in spitCoords(b):
    if dots.hasKey(c):
      if found == 0 or found > mh(c):
        found = mh(c)
  return found

const testData = """
R8,U5,L5,D3|U7,R6,D4,L4|6
R75,D30,R83,U83,L12,D49,R71,U7,L72|U62,R66,U55,R34,D71,R55,D58,R83|159
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51|U98,R91,D20,R16,D67,R40,U7,R15,U6,R7|135"""

for line in testData.splitLines():
  let parts = line.split('|')
  assert closest(parts[0], parts[1]) == parseInt(parts[2])

var strm = newFileStream("day03.data")
assert not isNil(strm)
let a = strm.readLine()
let b = strm.readLine()
strm.close()

echo closest(a, b)