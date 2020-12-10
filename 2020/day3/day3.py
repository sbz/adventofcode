"""
Advent of Code 2020: Day 3 Toboggan Trajectory

https://adventofcode.com/2020/day/3
"""
import copy
import sys

from typing import List, Union
from dataclasses import dataclass

@dataclass
class Matrix:
    array: List[list]
    x: int
    y: int

    def draw(self) -> str:
        result = ""
        for y in ["".join(x) for x in self.array]:
            result+=y + '\n'

        return result

def trajectory_part1(matrix: Matrix, right_step: int=3, down_step: int=1) -> int:
    """
    Part 1, 1 slope right 3, down 1, count trees
    """
    count = 0
    x, y = 0, 0
    matrix = copy.deepcopy(matrix)
    for i in range(1, matrix.y):
        x = (x + right_step) % matrix.x
        y += down_step
        if matrix.array[y][x] == '#':
            count += 1
            matrix.array[y][x] = 'X'
        elif matrix.array[y][x] == '.':
            matrix.array[y][x] = 'O'

        if y >= (matrix.y - 1):
            break

    return count


def trajectory_part2(matrix: Matrix) -> int:
    """
    Part 2, mutli slopes, count trees
    """
    result = 1
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    for x, y in  slopes:
        result *= trajectory_part1(matrix, x, y)

    return result

def solve(file_input: str):
    result = int()
    array =  list()
    x, y = 0, 0

    with open(file_input, 'r') as fd:
        for line in fd.readlines():
            l = line.strip()
            x = len(l)
            array.append(list(l))
            y += 1

    matrix = Matrix(array, x, y)

    print(matrix.draw())
    result = trajectory_part1(matrix)
    print("Result Part 1: {}".format(result))
    result = trajectory_part2(matrix)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    solve(sys.argv[1])
