#!/usr/bin/env nim c -r
import advent
import strutils
import re
import sequtils

type Coord = tuple[x: int, y: int, z: int]
type Body = tuple[pos: Coord, velocity: Coord]

proc toBodies(raw: string): seq[Body] =
  let parser = re(r"^\s*<\w=(-?\d+),\s*\w=(-?\d+),\s*\w=(-?\d+)>\s*$")
  var bodies: seq[Body]

  var nums: array[3, string]
  for line in raw.splitLines():
    let line = line.strip()
    if match(line, parser, nums):
      let v = (x: 0, y: 0, z: 0)
      let c = (x: parseInt(nums[0]), y: parseInt(nums[1]), z: parseInt(nums[2]))
      let b = (pos: c, velocity: v)
      bodies.add(b)

  return bodies

proc repeatFast(input: string): int64 =
  let orig = toBodies(input)

  proc calcPart(original: array[0 .. 3, int]): int =
    var pos: array[0 .. 3, int] = original
    var vel: array[0 .. 3, int] = [0, 0, 0, 0]

    var count = 0
    var d: int
    while true:
      count += 1
      for i in 0 .. 3:
        for j in 0 .. 3:
          if i == j:
            continue
          d = system.cmp(pos[i], pos[j])
          if d == -1:
            vel[i] += 1
            vel[j] -= 1
          elif d == -1:
            vel[i] -= 1
            vel[j] += 1
      for i in 0 .. 3:
        pos[i] += vel[i]
      if pos == original and all(vel, proc (a: int): bool = a == 0):
        return count

  var oneBody: array[0 .. 3, int] = [0, 0, 0, 0]
  for i in 0 .. 3:
    oneBody[i] = orig[i][0].x
  let x = calcPart(oneBody)
  for i in 0 .. 3:
    oneBody[i] = orig[i][0].y
  let y = calcPart(oneBody)
  for i in 0 .. 3:
    oneBody[i] = orig[i][0].z
  let z = calcPart(oneBody)

  echo (x, y, z)
  let m = max(max(x, y), z)
  var rtv: int64 = m
  while not (rtv %% x == 0 and rtv %% y == 0 and rtv %% z == 0):
    rtv += m
  echo rtv
  return rtv

let first = """
  <x=-1, y=0, z=2>
  <x=2, y=-10, z=-7>
  <x=4, y=-8, z=8>
  <x=3, y=5, z=-1>"""
assert repeatFast(first) == 2772

let second = """
  <x=-8, y=-10, z=0>
  <x=5, y=5, z=10>
  <x=2, y=-7, z=3>
  <x=9, y=-8, z=-3>"""
assert repeatFast(second) == 4686774924

let input = readInput("day12.data")
echo repeatFast(input)
