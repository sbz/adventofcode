"""
Advent of Code 2021: Day 1 Sonar Sweep

https://adventofcode.com/2021/day/1
"""
import sys

from typing import List, Union

def sonar_part1(array: List[int]) -> int:
    count = 0
    prev = array[0]
    for i in range(1, len(array)):
        if array[i] > prev:
            count += 1
        prev = array[i]

    return count

def sonar_part2(array: List[int]) -> int:
    count = 0
    a, b, c = array[0], array[1], array[2]
    prev_sum = a + b + c
    for i in range(0, len(array)-2):
        a = array[i]
        b = array[i+1]
        c = array[i+2]
        next_sum = a  + b + c
        if next_sum > prev_sum:
            count += 1
        prev_sum = next_sum

    return count


def solve(file_input: str = './input.txt'):
    result = int()
    array = list()

    with open(file_input, 'r') as fd:
        nums = fd.readlines()
        array = [int(x.strip()) for x in nums]

    result = sonar_part1(array)
    print("Result Part 1: {}".format(result))
    result = sonar_part2(array)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    if len(sys.argv) == 1:
        solve()
    else:
        solve(sys.argv[1])
