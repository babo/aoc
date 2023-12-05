#!/usr/bin/env python3
import sys

def map_range(value, rule):
    a, b, v = value
    x, y, d = rule
    if b <= x or a >= y:
        return [value]
    if a < x and b > y:
        return [(a, x, 0), (x, y, d), (y, b, 0)]
    if a < x:
        return [(a, x, 0), (x, b, d)]
    if b > y:
        return [(a, y, d), (y,  b, 0)]
    return  [(a, b, d)]

def main(name):
    with open(name) as fd:
        seeds = []
        prev = None
        for i, x in enumerate([int(x) for x in fd.readline().strip()[6:].split()]):
            if i % 2 == 0:
                prev = x
            else:
                seeds.append((prev, prev + x, 0))

        mappings = []
        fd.readline()
        for line in fd.readlines():
            line = line.strip()
            if line.endswith(" map:"):
                print(line)
                mappings = []
            elif line:
                nv, start, width = tuple(int(x) for x in line.split())
                mappings.append((start, start + width, nv - start))
            else:
                nseeds = []
                for value in seeds:
                    for rule in mappings:
                        ns = map_range(value, rule)
                        for v in ns:
                            nseeds.append(v)
                coll = {}
                for x in sorted(nseeds):
                    tba = []
                    for k in coll:
                        if k[0] >= x[1] or k[1] <= x[0]:
                            continue
                        l = min(x[0], k[0])
                        r = min(x[1], k[1])
                        tba.append((l, r, x[2]))
                        x = (r, x[1], x[2])
                        if r == x[1]:
                            break
                    if x[0] != x[1]:
                        tba.append(x)
                    for (a, b, c) in tba:
                        if (a, b) not in coll:
                            coll[(a, b)] = c
                        else:
                            coll[(a, b)] += c
                seeds = [(k[0] + coll[k], k[1] + coll[k], 0) for k in coll]

        seeds = [(l + d, h + d, 0) for (l, h, d) in seeds]
        v = [x[0] for x in seeds]
        print(min(v))

if __name__ == '__main__':
    main(sys.argv[1])
