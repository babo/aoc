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

def validate(left, right):
    for i in range(len(left)):
        if i >= len(right):
            return False, False
        elif isinstance(left[i], int) and isinstance(right[i], int):
            if left[i] == right[i]:
                continue
            return left[i] < right[i], False
        elif isinstance(left[i], int):
            left[i] = [left[i]]
        elif isinstance(right[i], int):
            right[i] = [right[i]]

        valid, cont = validate(left[i], right[i])
        if not valid or not cont:
            return valid, False
    return True, len(left) == len(right)

def step_1():
    n = 0
    packets = all_packets()
    for i, lr in enumerate(zip(packets[0::2], packets[1::2])):
        v, _ = validate(lr[0].line, lr[1].line)
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
