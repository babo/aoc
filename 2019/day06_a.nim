#!/usr/bin/env nim c -r
import options
import streams
import strutils
import tables

var testData = """ COM)B
G)H
D)I
E)J
B)C
C)D
D)E
E)F
B)G
J)K
K)L"""

proc checksum(data: string): int =
  var known: Table[string, Option[string]];

  for line in splitLines(data):
    let ft = line.strip().split(')')
    if not known.hasKey(ft[0]):
      known[ft[0]] = none(string)
    known[ft[1]] = some(ft[0])

  var n: Option[string]
  var w, weight = 0
  for v in values(known):
    w = 0
    n = v
    while n.isSome():
      w += 1
      n = known[n.get()]
    weight += w
  return weight

assert checksum(testData) == 42

var strm = newFileStream("day06.data")
assert not isNil(strm)
let input = strm.readAll()
strm.close()

echo checksum(input)