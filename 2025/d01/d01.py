#!/usr/bin/env uv run --script


def p3(pos: int, count: int, line: str):
    right = line.startswith("R")
    r = int(line[1:])
    inc = 0
    if r >= 100:
        inc = r // 100
        r %= 100
    if right:
        n = pos + r
        if n >= 100:
            inc += 1
    else:
        if pos > r:
            n = pos - r
        else:
            if pos != 0:
                inc += 1
            n = 100 + pos - r
    return n % 100, count + inc


def main():
    s = 50
    t = 0
    with open("input") as fd:
        for line in [x.strip() for x in fd.readlines() if x.strip()]:
            s, t = p3(s, t, line)
        print(t)


if __name__ == "__main__":
    main()
