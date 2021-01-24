#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils

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
  try:
    return map[0][a[0] + map[1] * a[1]]
  except:
    return false

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

let map_02_raw = """
  ......#.#.
  #..#.#....
  ..#######.
  .#.#.###..
  .#..#.....
  ..#....#.#
  #..#....#.
  .##.#..###
  ##...#..#.
  .#....####
"""
let map_02 = toMap(map_02_raw)

assert num_visible(map_02, (5, 8)) == 33

assert best_pos(map_02) == (5, 8, 33)

let map_04_raw = """
  ..........
  ..........
  ..........
  .....b....
  ..........
  ...c.@.d..
  ..........
  .....e....
  ..........
  ..........
"""
let map_04 = toMap(map_04_raw)

assert atMap(map_04, (3, 5))
assert atMap(map_04, (7, 5))
assert atMap(map_04, (5, 3))
assert atMap(map_04, (5, 5))
assert atMap(map_04, (5, 7))

for dp in [(-1, 0), (1, 0), (0, 1)]:
  var dp = dp
  assert visible(map_04, (5, 5), dp) == false
  dp = (dp[0] * 2, dp[1] * 2)
  assert visible(map_04, (5, 5), dp) == true
  dp = (dp[0] * 2, dp[1] * 2)
  assert visible(map_04, (5, 5), dp) == false

let map_01_raw = """
  #.........
  ...A......
  ...B..a...
  .EDCG....a
  ..F.c.b...
  .....c....
  ..efd.c.gb
  .......c..
  ....f...c.
  ...e..d..c
"""
let line_01 = join(map_01_raw.splitLines().map(proc (x: string): string = x.strip()), "")
let map_01 = toMap(map_01_raw)

assert map_01[1] == 10
assert map_01[2] == 10
assert map_01[0].len == 100

proc mapc(map: Map, a: (int, int)): char = line_01[a[0] + map[1] * a[1]]

for x in (0 .. map_01[1] - 1):
  for y in (0 .. map_01[2] - 1):
    let p = (x, y)
    let c = mapc(map_01, p)
    let r = visible(map_01, (0, 0), p)
    if c == '.':
      assert r == false
    if c == '#':
      assert r == false
    if c >= 'a' and c <= 'z':
      assert r == false
    if c >= 'A' and c <= 'Z':
      assert r == true

let map_03_raw = """
  #.#...#.#.
  .###....#.
  .#....#...
  ##.#.#.#.#
  ....#.#.#.
  .##..###.#
  ..#...##..
  ..##....##
  ......#...
  .####.###.
"""
let map_03 = toMap(map_03_raw)
assert num_visible(map_03, (1, 2)) == 35
assert best_pos(map_03) == (1, 2, 35)

let map_05_raw = """
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
let map_05 = toMap(map_05_raw)
assert best_pos(map_05) == (11, 13, 210)

let real_map = toMap(readInput("day10.data"))
echo best_pos(real_map)