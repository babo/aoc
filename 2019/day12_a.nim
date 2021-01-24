#!/usr/bin/env nim c -r
import advent
import strutils
import re

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

proc step(bodies: seq[Body]): seq[Body] =
  var bodies = bodies
  let n = bodies.len - 1

  for i in 0 .. n:
    let bp = bodies[i][0]
    var bv = bodies[i][1]
    for j in 0 .. n:
      if j == i:
        continue
      let other = bodies[j][0]
      bv.x += system.cmp(other.x, bp.x)
      bv.y += system.cmp(other.y, bp.y)
      bv.z += system.cmp(other.z, bp.z)
    bodies[i] = (bp, bv)
  for i in 0 .. n:
    var bp = bodies[i][0]
    let bv = bodies[i][1]
    bp.x += bv.x
    bp.y += bv.y
    bp.z += bv.z
    bodies[i] = (bp, bv)
  return bodies

proc energy(bodies: seq[Body]): int =
  var total = 0
  var pot = 0
  var kin = 0
  for b in bodies:
    pot = abs(b[0].x) + abs(b[0].y) + abs(b[0].z)
    kin = abs(b[1].x) + abs(b[1].y) + abs(b[1].z)
    total += pot * kin
  return total

proc energyAfter(input: string, steps: int): int =
  var tb = toBodies(input)
  for i in 1 .. steps:
    tb = step(tb)
  return energy(tb)

let first = """
  <x=-1, y=0, z=2>
  <x=2, y=-10, z=-7>
  <x=4, y=-8, z=8>
  <x=3, y=5, z=-1>"""
assert energyAfter(first, 10) == 179

let second = """
  <x=-8, y=-10, z=0>
  <x=5, y=5, z=10>
  <x=2, y=-7, z=3>
  <x=9, y=-8, z=-3>"""
assert energyAfter(second, 100) == 1940

let input = readInput("day12.data")
echo energyAfter(input, 1000)
