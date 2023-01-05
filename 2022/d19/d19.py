#!/usr/bin/env python3
import itertools
import re


def simulate(tmax, costs, phase_1=None, phase_2=None):
    if phase_1 and phase_1.endswith("*"):
        phase_1 = phase_1[:-2] + phase_1[-2] * tmax
    if phase_2 and phase_2.endswith("*"):
        phase_2 = phase_2[:-2] + phase_2[-2] * tmax

    p = 0
    r = [1, 0, 0, 0]
    n = [0, 0, 0, 0]
    phase_one = True
    rtv = []

    for _ in range(tmax):
        cond = [
            n[0] >= costs[0][0] and n[1] >= costs[0][1] and n[2] >= costs[0][2],
            n[0] >= costs[1][0] and n[1] >= costs[1][1] and n[2] >= costs[1][2],
            n[0] >= costs[2][0] and n[1] >= costs[2][1] and n[2] >= costs[2][2],
            n[0] >= costs[3][0] and n[1] >= costs[3][1] and n[2] >= costs[3][2],
        ]

        prev = list(r)
        if phase_one and (cond[2] or cond[3]):
            phase_one = False
            p = 0

        if phase_one:
            if p < len(phase_1):
                kind = int(phase_1[p]) - 1
                for i in range(3):
                    if costs[kind][i] > n[i]:
                        break
                else:
                    p += 1
                    r[kind] += 1
                    for i in range(3):
                        n[i] -= costs[kind][i]
            else:
                p = 0
                phase_one = False

        if phase_one == False:
            if cond[3]:
                kind = 3
            elif cond[2]:
                kind = 2
            elif cond[1]:
                kind = 1
            else:
                kind = None

            if kind and p < len(phase_2) and not cond[3]:
                kind = int(phase_2[p]) - 1
                if (cond[1] and kind == 1) or (cond[2] and kind == 2):
                    p += 1
                else:
                    kind = None
            if kind is not None:
                r[kind] += 1
                for i in range(3):
                    n[i] -= costs[kind][i]
                    assert n[i] >= 0, "Not enough juice"

        for i in range(4):
            n[i] += prev[i]
        rtv.append(list(n))
    return rtv


def combinations(n=6):
    s = set()
    for x in itertools.combinations_with_replacement("23", n):
        for x in itertools.permutations(x):
            n = "".join(x)
            if n.endswith("2"):
                s.add(n + "3*")
            else:
                p = n.rfind("2")
                if p == -1:
                    s.add("3*")
                else:
                    s.add(n[:p] + "23*")
    return list(s)


def prepare(costs, tmax=32):
    m = 0
    solution = None
    for phase_2 in combinations():
        for phase_1 in combinations():
            phase_1 = phase_1.replace("2", "1").replace("3", "2")
            v = simulate(tmax, costs, phase_1, phase_2)[-1][3]
            if v > m:
                solution = (phase_1, phase_2)
            m = max(m, v)
    print(solution)
    return m


def costs(line):
    n = int(re.match(r"^Blueprint (\d+):", line).group(1))
    ore = re.match(r".*Each ore robot costs (\d+) ore", line).group(1)
    clay = re.match(r".*Each clay robot costs (\d+) ore", line).group(1)
    obsidian = re.match(r".*Each obsidian robot costs (\d+) ore and (\d+) clay", line).groups()
    geode = re.match(r".*Each geode robot costs (\d+) ore and (\d+) obsidian", line).groups()
    return [(int(ore), 0, 0), (int(clay), 0, 0), (int(obsidian[0]), int(obsidian[1]), 0, 0), (int(geode[0]), 0, int(geode[1]), 0)]


def main():
    t1 = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."
    t2 = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
    assert prepare(costs(t1), 32) == 56, "It should reach 56"
    assert prepare(costs(t2), 32) == 62, "It should reach 62"

    with open("input.txt") as fd:
        n = 1
        m = 0
        for line in fd.readlines():
            cost = costs(line)
            v = prepare(cost, 24)
            m += n * v
            n += 1
    assert m == 2193

    with open("input.txt") as fd:
        m = 1
        for line in fd.readlines()[:3]:
            cost = costs(line)
            v = prepare(cost, 32)
            print(v)
            m *= v
        assert m == 7200


if __name__ == "__main__":
    main()
