"""
Advent of Code 2021: Day 8 Seven Segment Search

https://adventofcode.com/2021/day/8
"""
import sys

from collections import defaultdict
from itertools import permutations
from pprint import pprint
from typing import List

def is_unique_segment(out: str) -> bool:
    return len(out) in [2, 3, 4, 7]

def segment_part1(array: List[str]) -> int:
    count = 0

    for _, outputs in array:
        for out in outputs.split():
            if is_unique_segment(out):
                count += 1

    return count

def segment_part2(array: List[str]) -> int:
    values = list()

def solve(file_input: str = './input.txt'):
    result = int()
    signals = list()

    with open(file_input, 'r') as fd:
        for line in fd:
            patterns, outputs = line.split(" | ")
            signals.append((patterns.strip(), outputs.strip()))

    result = segment_part1(signals)
    print("Result Part 1: {}".format(result))
    result = segment_part2(signals)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    if len(sys.argv) == 1:
        solve()
    else:
        solve(sys.argv[1])
