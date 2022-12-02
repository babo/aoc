#!/usr/bin/env python3
import sys

# Rock A X
# Paper B Y
# Scissor C Z

score = {
    "A X": 4,
    "A Y": 8,
    "A Z": 3,
    "B X": 1,
    "B Y": 5,
    "B Z": 9,
    "C X": 7,
    "C Y": 2,
    "C Z": 6
}

with open(sys.argv[1]) as fd:
    print(sum([score[line.strip()] for line in fd.readlines()]))
