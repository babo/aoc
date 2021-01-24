#!/usr/bin/env nim c -r
import options
import streams
import strutils
import tables

var testData = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"""

proc checksum(data: string): int =
  var known: Table[string, Option[string]];

  for line in splitLines(data):
    let ft = line.strip().split(')')
    if not known.hasKey(ft[0]):
      known[ft[0]] = none(string)
    known[ft[1]] = some(ft[0])

  proc parents(name: string): seq[string] =
    var l: seq[string] = @[]
    var n = some(name)
    while known[n.get()].isSome():
      l.add(n.get())
      n = known[n.get()]
    return l

  var you = parents("YOU")
  var san = parents("SAN")
  while you.pop() == san.pop():
    discard

  return you.len() + san.len()

assert checksum(testData) == 4

var strm = newFileStream("day06.data")
assert not isNil(strm)
let input = strm.readAll()
strm.close()

echo checksum(input)