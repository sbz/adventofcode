"""
Advent of Code 2020: Day 1 Report Repair

https://adventofcode.com/2020/day/1
"""
import sys

from typing import List, Union


def sumup_part1(array: List[int], total: int=2020) -> Union[int, None]:
    """
    Part 1

    Find two numbers that sump up to 2020
    """
    s=total
    input_set=set(array)
    for x in input_set:
        if s-x in input_set and x <= s-x:
            result = x * (s-x)
            return result

    return None

def sumup_part2(array: List[int], total: int=2020) -> Union[int, None]:
    """
    Part 2

    Find three numbers that sum up to 2020
    """
    size = len(array)
    for i in range(0, size-1):
        input_set = set()
        cur_sum = total - array[i]
        for j in range(i + 1, size):
            if cur_sum - array[j] in input_set:
                result = array[i] * array[j] * (cur_sum-array[j])
                return result
            input_set.add(array[j])

    return None


def solve(file_input: str):
    result = int()
    array = list()

    with open(file_input, 'r') as fd:
        nums = fd.readlines()
        array = [int(x.strip()) for x in nums]

    result = sumup_part1(array)
    print("Result Part 1: {}".format(result))
    result = sumup_part2(array)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    solve(sys.argv[1])
