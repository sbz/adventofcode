"""
Advent of Code 2021: Day 6 Lanternfish

https://adventofcode.com/2021/day/6
"""
import sys

from dataclasses import dataclass
from typing import List

def lantern_slow_part1(array: List, days=80) -> int:
    nb_days = 1

    print("Initial State:", array)

    while nb_days <= days:
        for i in range(len(array)):
            if array[i] == 0:
                array[i] = 6
                array.append(8)
            else:
                array[i] += -1
        print("After {:02d} days: {}".format(nb_days, array))
        nb_days += 1

    return len(array)

def lantern_part1(array: List, days=80) -> int:
    fish = [0] * 9

    def next_day(fish):
        dp = [0] * 9

        dp[8] += fish[0]
        dp[6] += fish[0]

        for i in range(1, 9):
            dp[i - 1] += fish[i]

        return dp

    for n in array:
        fish[n] += 1

    for i in range(days):
        fish = next_day(fish)

    return sum(fish)

def lantern_part2(array: List, days=256) -> int:
    return lantern_part1(array, days)

def solve(file_input: str = './input.txt'):
    result = int()
    array = list()

    with open(file_input, 'r') as fd:
        for line in fd:
            array = [int(x) for x in line.strip().split(",")]

    #result = lantern_slow_part1(array)
    #print("Result Part 1: {}".format(result))
    result = lantern_part1(array, 80)
    print("Result Part 1: {}".format(result))
    result = lantern_part2(array, 256)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    if len(sys.argv) == 1:
        solve()
    else:
        solve(sys.argv[1])
