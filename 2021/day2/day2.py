"""
Advent of Code 2021: Day 2 Dive

https://adventofcode.com/2021/day/2
"""
import sys

from typing import List, Union

def dive_part1(array: List[int]) -> int:
    horizontal = 0
    depth = 0

    for command, value in array:
        if command == "forward":
            horizontal += value
        elif command == "up":
            depth -= value
        elif command == "down":
            depth += value

    return horizontal * depth

def dive_part2(array: List[int]) -> int:
    horizontal = 0
    depth = 0
    aim = 0

    for command, value in array:
        if command == "forward":
            horizontal += value
            depth += value * aim
        elif command == "up":
            aim -= value
        elif command == "down":
            aim += value

    return horizontal * depth

def solve(file_input: str = './input.txt'):
    result = int()
    array = list()

    with open(file_input, 'r') as fd:
        for line in fd:
            command, value = line.strip().split()
            array.append((command, int(value)))

    result = dive_part1(array)
    print("Result Part 1: {}".format(result))
    result = dive_part2(array)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    if len(sys.argv) == 1:
        solve()
    else:
        solve(sys.argv[1])
