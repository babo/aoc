#!/usr/bin/env python3
import sys

def part_a():
    assert len(sys.argv) != 1
    with open(sys.argv[1]) as fd:
        lines = [line.strip() for line in fd.readlines() if line.strip()]
        rows = len(lines)
        cols = len(lines[0])
        data = list(''.join(lines))
        start = data.index('S')
        goal = data.index('E')
        data[start] = 'a'
        data[goal] = 'z'
        start = start // cols, start % cols
        goal = goal // cols, goal % cols

    get_value = lambda rc: ord(data[rc[0]*cols + rc[1]])
    valid = lambda rc: rc[0] >= 0 and rc[0] < rows and rc[1] >= 0 and rc[1] < cols

    seen = dict()
    visit = {(start, 0)}
    found = None

    while visit:
        round = list(visit)
        visit = set()
        for (p, count) in round:
            v = get_value(p)
            if p not in seen or seen[p] > count:
                seen[p] = count
            if p == goal and (found is None or found > count):
                found = count
            for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                n = p[0] + dr, p[1] + dc
                if valid(n) and get_value(n) <= v + 1 and n not in seen:
                    visit.add((n, count+1))

    print(found)
    print(seen[goal])

def part_b():
    assert len(sys.argv) != 1
    with open(sys.argv[1]) as fd:
        lines = [line.strip() for line in fd.readlines() if line.strip()]
        rows = len(lines)
        cols = len(lines[0])
        data = list(''.join(lines))
        start = data.index('S')
        goal = data.index('E')
        data[start] = 'a'
        data[goal] = 'z'
        start = start // cols, start % cols
        goal = goal // cols, goal % cols

    get_value = lambda rc: ord(data[rc[0]*cols + rc[1]])
    valid = lambda rc: rc[0] >= 0 and rc[0] < rows and rc[1] >= 0 and rc[1] < cols

    found = None
    seen = dict()
    visit = set()

    for r in range(rows):
        for c in range(cols):
            if get_value((r, c)) == ord('a'):
                visit.add(((r, c), 0))

    while visit:
        round = list(visit)
        visit = set()
        for (p, count) in round:
            v = get_value(p)
            if p not in seen or seen[p] > count:
                seen[p] = count
            if p == goal and (found is None or found > count):
                found = count
            for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                n = p[0] + dr, p[1] + dc
                if valid(n) and get_value(n) <= v + 1 and (n not in seen or seen[n] > count):
                    visit.add((n, count+1))

    print(found)
    print(seen[goal])

def main():
    part_b()

if __name__ == '__main__':
    main()
