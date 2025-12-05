#!/usr/bin/env python3
from operator import itemgetter


def main():
    with open("input") as fd:
        data = [
            [int(x) for x in line.strip().split("-")]
            for line in fd.readlines()
            if "-" in line
        ]
        data.sort(key=itemgetter(1))
        data.sort(key=itemgetter(0))
        n = 0
        lo, hi = data[0]
        for r in data[1:]:
            if hi < r[0]:
                n += 1 + hi - lo
                lo, hi = r
            elif hi <= r[1]:
                hi = r[1]
        n += 1 + hi - lo
        assert n == 347468726696961


if __name__ == "__main__":
    main()
