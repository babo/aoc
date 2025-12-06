#!/usr/bin/env python3


def main():
    with open("simple.txt") as fd:
        lines = [line for line in fd.read().splitlines() if line.strip()]
        m = max([len(x) for x in lines])

    op, total, st = None, 0, 0
    for c in range(m):
        n = 0
        for r in lines[:-1]:
            if c < len(r) and r[c].isdigit():
                n = n * 10 + int(r[c])
        if n == 0:
            total += st
            op, st = None, 0
        elif op is None:
            st = n
            match lines[-1][c]:
                case "+":
                    op = "+"
                case "*":
                    op = "*"
                case _:
                    assert f"Unknown {lines[-1][c]}"
        else:
            match op:
                case "+":
                    st += n
                case "*":
                    st *= n
    total += st
    print(total)
    assert total == 3263827


if __name__ == "__main__":
    main()
