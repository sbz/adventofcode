"""
Advent of Code 2021: Day 7 The Treachery of Whales

https://adventofcode.com/2021/day/7
"""
import sys

from typing import List

def crab_part1(array: List) -> int:
    def fuel_count(positions: List, align: int) -> int:
        total_fuel = 0

        for pos in positions:
            fuel = abs(align-pos)
            total_fuel += fuel

        return total_fuel

    result = sys.maxsize

    for pos in range(min(array), max(array)):
        fuel_at_pos = fuel_count(array, pos)
        result = min(result, fuel_at_pos)

    return result

def crab_part2(array: List) -> int:
    def fuel_count(positions: List, align: int) -> int:
        total_fuel = 0

        for pos in positions:
            n = abs(align-pos)
            fuel = (n*(n+1))/2 # gauss arithmetic progression
            total_fuel += fuel

        return int(total_fuel)

    result = sys.maxsize

    for pos in range(min(array), max(array)):
        fuel_at_pos = fuel_count(array, pos)
        result = min(result, fuel_at_pos)

    return result

def solve(file_input: str = './input.txt'):
    result = int()
    array = list()

    with open(file_input, 'r') as fd:
        for line in fd:
            array = [int(x) for x in line.strip().split(",")]

    result = crab_part1(array)
    print("Result Part 1: {}".format(result))
    result = crab_part2(array)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    if len(sys.argv) == 1:
        solve()
    else:
        solve(sys.argv[1])
