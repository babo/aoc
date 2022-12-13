#!/usr/bin/env python3
import sys

class Packet:
    def __init__(self, line):
        self.line = eval(line)
        self.divider = line == '[[2]]' or line == '[[6]]'

    def __lt__(self, other):
        return validate(self.line, other.line)[0]

def all_packets():
    with open(sys.argv[1]) as fd:
        return [Packet(line.strip()) for line in fd.readlines() if line.strip()]

def pairs():
    with open(sys.argv[1]) as fd:
        while True:
            left = fd.readline()
            right = fd.readline()
            if left and right:
                yield (left.strip(), right.strip())
                fd.readline()
            else:
                return

def validate(left, right):
    assert isinstance(left, list) and isinstance(right, list)

    for i in range(len(left)):
        if i >= len(right):
            return False, False
        if isinstance(left[i], int) and isinstance(right[i], int):
            if left[i] == right[i]:
                continue
            return left[i] < right[i], False
        if isinstance(left[i], int):
            #if i+1 != len(left):
            #    return False, False
            n = left[i]
            left[i] = list()
            left[i].append(n)
        elif isinstance(right[i], int):
            #if i+1 != len(right):
            #    return False, False
            n = right[i]
            right[i] = list()
            right[i].append(n)

        val, cont = validate(left[i], right[i])
        if not val or not cont:
            return val, False
    return True, len(left) == len(right)

def step_1():
    n = 0
    for i, lr in enumerate(pairs()):
        a = eval(lr[0])
        b = eval(lr[1])
        v = validate(a, b)[0]
        if v:
            n += i+1
    print(n)

def step_2():
    packets = all_packets()
    packets.append(Packet('[[2]]'))
    packets.append(Packet('[[6]]'))
    packets.sort()
    n = 1
    for i, p in enumerate(packets):
        if p.divider:
            n *= i+1
    print(n)

def main():
    step_1()
    step_2()

if __name__ == '__main__':
    main()
