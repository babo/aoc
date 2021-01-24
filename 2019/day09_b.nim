#!/usr/bin/env nim c -r
import advent
import sequtils
import strutils

let input = readInput("day09.data").split(',').map(parseBiggestInt)

runIntCode(input, proc (): int64 = 2, proc (x: int64) = echo x)