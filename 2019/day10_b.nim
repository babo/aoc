#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils
import algorithm

type Map = (seq[bool], int, int)

proc toMap(input: string): Map =
  var x = -1
  var y = 0
  var map: seq[bool] = @[]

  for line in input.splitLines():
    let content = line.strip()
    if content.len == 0:
      continue
    if x < 0:
      x = content.len
    else:
      assert x == content.len
    y += 1
    for a in content:
      map.add(a != '.')
  return (map, x, y)

proc atMap(map: Map, a: (int, int)): bool =
  return map[0][a[0] + map[1] * a[1]]

proc one_step(a: (int, int)): (int, int) =
  var b = a

  let limit = if a[0] * a[1] != 0: min(abs(a[0]), abs(a[1])) else: max(abs(a[0]), abs(a[1]))
  for i in (2 .. limit):
    while b[0] mod i == 0 and b[1] mod i == 0:
      b = (b[0] div i, b[1] div i)

  return b

proc visible(map: Map, a: (int, int), b: (int, int)): bool =
  if b == (0, 0):
    return false
  let delta = one_step(b)
  var step = delta

  while step != b:
    if atMap(map, (a[0] + step[0], a[1] + step[1])):
      return false
    step[0] += delta[0]
    step[1] += delta[1]

  return atMap(map, (a[0] + step[0], a[1] + step[1]))

proc num_visible(map: Map, a: (int, int)): int =
  var c = 0
  let (x, y) = a
  if atMap(map, (x, y)):
    for xx in (0 .. map[1] - 1):
      for yy in (0 .. map[2] - 1):
        if visible(map, (x, y), (xx - x, yy - y)):
          c += 1
  return c

proc best_pos(map: Map): (int, int, int) =
  var m = (-1, -1, 0)
  for x in (0 .. (map[1] - 1)):
    for y in (0 .. (map[2] - 1)):
      let c = num_visible(map, (x, y))
      if c > m[2]:
        m = (x, y, c)
  return m

proc planeNum(a: (int, int)): int =
  let (x, y) = a
  if x > 0 and y >= 0:
    return 1
  if x >= 0 and y < 0:
    return 0
  return if y <= 0 : 3 else: 2

assert planeNum((10, 0)) == 1
assert planeNum((10, 2)) == 1
assert planeNum((-10, 10)) == 2
assert planeNum((0, 2)) == 2

assert planeNum((-2, 0)) == 3
assert planeNum((-2, -10)) == 3
assert planeNum((0, -10)) == 0
assert planeNum((2, -10)) == 0

proc toPlane0(a: (int, int)): (int, int) =
  case planeNum(a)
    of 1: return a
    of 2: return (a[1], -a[0])
    of 3: return (-a[0], -a[1])
    of 0: return (-a[1], a[0])
    else: assert false

assert toPlane0((2, 4)) == (2, 4)
assert toPlane0((4, -2)) == (2, 4)
assert toPlane0((-2, -4)) == (2, 4)
assert toPlane0((-4, 2)) == (2, 4)

proc squared(a: (int, int)): int = a[0] * a[0] + a[1] * a[1]

proc cmp(a: (int, int), b: (int, int)): int =
  var rtv = system.cmp(planeNum(a), planeNum(b))
  if  rtv != 0:
    return rtv

  let a = toPlane0(a)
  let b = toPlane0(b)
  let r = system.cmp(a[1] * b[0], a[0] * b[1])
  return if r != 0: r else: system.cmp(squared(a), squared(b))

assert cmp((1, 1), (2, 2)) == -1
assert cmp((0, 1), (0, 2)) == -1
assert cmp((2, 2), (3, 5)) == -1
assert cmp((1, 1), (1, 2)) == -1
assert cmp((1, 1), (1, -1)) == 1
assert cmp((0, 1), (-1, 1)) == -1

proc vaporize(map: Map): seq[(int, int)] =
  var map: Map = map
  let bp = best_pos(map)
  let c:(int, int) = (bp[0], bp[1])
  var dx: int
  var dy: int

  var rtv: seq[(int, int)]
  while true:
    var items: seq[(int, int)]
    for x in (0 .. map[1] - 1):
      for y in  (0 .. map[2] - 1):
        if atMap(map, (x, y)):
          dx = x - c[0]
          dy = y - c[1]
          if dx == 0 and dy == 0:
            discard
          elif visible(map, c, (dx, dy)):
            items.add((dx, dy))
    for p in sorted(items, cmp, Ascending).map(proc (a: (int, int)): (int, int) = (c[0] + a[0], c[1] + a[1])):
      rtv.add(p)
      map[0][p[0] + map[1] * p[1]] = false
    if items.len == 0:
      break

  return rtv

let test_map_raw = """
  .#..##.###...#######
  ##.############..##.
  .#.######.########.#
  .###.#######.####.#.
  #####.##.#.##.###.##
  ..#####..#.#########
  ####################
  #.####....###.#.#.##
  ##.#################
  #####.##.###..####..
  ..######..##.#######
  ####.##.####...##..#
  .#####..#.######.###
  ##...#.##########...
  #.##########.#######
  .####.#.###.###.#.##
  ....##.##.###..#####
  .#.#.###########.###
  #.#.#.#####.####.###
  ###.##.####.##.#..##
"""
let test_map = toMap(test_map_raw)
let test_result = vaporize(test_map)
echo test_result.len
assert test_result.len == 299
for d in [(1, 11, 12), (2, 12, 1), (3, 12, 2), (10, 12, 8), (20, 16, 0), (50, 16, 9), (100, 10, 16), (199, 9, 6), (200, 8, 2), (201, 10, 9), (299, 11, 1)]:
  echo test_result[d[0] - 1], " <-> ", d
  assert test_result[d[0] - 1] == (d[1], d[2])

let real_map = toMap(readInput("day10.data"))
let real_order = vaporize(real_map)
echo real_order[199]
