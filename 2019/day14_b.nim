#!/usr/bin/env nim c -r
import advent
import strutils
import re
import options
import tables

# Part Two
let cargo = 1000000000000'i64

proc readReactions(raw: string): Table[string, (int, seq[(string, int)])] =
  let parser = re(r"(\d+) (\w+)")
  var table = initTable[string, (int, seq[(string, int)])]()

  var found: Option[(string, int)]
  var ingredients: seq[(string, int)]
  for line in raw.splitLines():
    found = none((string, int))
    ingredients = @[]
    let line = line.strip()
    for x in re.findAll(line, parser, 0):
      if found.isSome():
        ingredients.add(found.get())
      var parts = x.split(' ')
      found = some((parts[1], parseInt(parts[0])))
    if found.isSome():
      let name: string = found.get()[0]
      let quant: int = found.get()[1]
      table.add(name, (quant, ingredients))

  return table

proc makeFuel(recipe: Table[string, (int, seq[(string, int)])], stocks: Table[string, int64], needed: int64 = 1): (int64, Table[string, int64]) =
  var stocks: Table[string, int64] = stocks

  proc getPart(name: string, quant: int64): int64 =
    if name == "ORE":
      return quant

    assert recipe.hasKey(name)
    let (produce, ingredients) = recipe[name]

    var sum: int64 = 0
    if stocks.getOrDefault(name, 0) < quant:
      let n = quant - stocks.getOrDefault(name, 0)
      var times = n div produce
      if n mod produce != 0:
        times += 1
      for p in ingredients:
        sum += getPart(p[0], times * p[1])
      stocks[name] = stocks.getOrDefault(name, 0) + times * produce

    stocks[name] = stocks.getOrDefault(name, 0) - quant
    return sum

  let rtv = getPart("FUEL", needed)
  return (rtv, stocks)

proc allParts(input: string): int64 =
  let emptyStock = initTable[string, int64]()
  let one = makeFuel(readReactions(input), emptyStock)[0]

  var able: int64 = 0
  var total: int64 = 0
  var diff: int64 = cargo div one
  while total < cargo and diff > 0:
    able += diff
    echo (able, total, diff)
    total = makeFuel(readReactions(input), emptyStock, able)[0]
    diff = (cargo - total) div one
  return able

proc couldProduce(name: string): int64 = allParts(name)

let test3 = """
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
"""
assert couldProduce(test3) == 82892753'i64

let test4 = """
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
"""
assert couldProduce(test4) == 5586022'i64

let test5 = """
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
"""
assert couldProduce(test5) == 460664'i64

echo couldProduce(readInput("day14.data"))
