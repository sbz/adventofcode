"""
Advent of Code 2021: Day 5 Hydrothermal Venture

https://adventofcode.com/2021/day/5
"""
import sys

from dataclasses import dataclass
from pprint import pprint
from typing import List, Union, Tuple

@dataclass
class Matrix:
    data: List[List[int]]
    rows: int
    cols: int

def draw_vertical_line(matrix: Matrix, x: int, y1: int, y2: int):
    if y1 > y2:
        y1, y2 = y2, y1

    for y in range(y1, y2+1):
        matrix.data[y][x] += 1

def draw_horizontal_line(matrix: Matrix, y: int, x1: int, x2: int):
    if x1 > x2:
        x1, x2 = x2, x1

    for x in range(x1, x2+1):
        matrix.data[y][x] += 1

def draw_line(matrix: Matrix, x1: int, y1: int, x2: int, y2: int):
    sign_x = 1 if x1 < x2 else -1
    sign_y = 1 if y1 < y2 else -1

    distance = max(abs(x2-x1), abs(y2-y1))

    for i in range(0, distance+1):
        x = x1 + sign_x * i
        y = y1 + sign_y * i
        matrix.data[y][x] += 1

def dump_matrix(matrix: Matrix):
    for row in range(matrix.rows):
        for col in range(matrix.cols):
            sys.stdout.write('.' if not matrix.data[row][col] \
                                else str(matrix.data[row][col]))
        sys.stdout.write("\n")

def venture_part1(array: List, matrix_size: int) -> int:
    result = 0
    rows = cols = matrix_size
    data = [cols*[0] for _ in range(rows)]
    matrix = Matrix(data, rows, cols)

    for x1, y1, x2, y2 in array:
        if x1 == x2 and abs(x2-x1) < cols:
            draw_vertical_line(matrix, x1, y1, y2)
        if y1 == y2 and abs(y2-y2) < rows:
            draw_horizontal_line(matrix, y1, x1, x2)

    for row in range(matrix.rows):
        for col in range(matrix.cols):
            if matrix.data[row][col] > 1:
                result += 1

    dump_matrix(matrix)

    return result

def venture_part2(array: List, matrix_size: int) -> int:
    result = 0
    rows = cols = matrix_size
    data = [cols*[0] for _ in range(rows)]
    matrix = Matrix(data, rows, cols)

    for x1, y1, x2, y2 in array:
        if x1 == x2 and abs(x2-x1) < cols:
            draw_vertical_line(matrix, x1, y1, y2)
        if y1 == y2 and abs(y2-y1) < rows:
            draw_horizontal_line(matrix, y1, x1, x2)

    for x1, y1, x2, y2 in array:
        if x1 != x2 and y1 != y2 and abs(x2-x1) < cols and abs(y2-y1) < rows:
            draw_line(matrix, x1, y1, x2, y2)

    for row in range(matrix.rows):
        for col in range(matrix.cols):
            if matrix.data[row][col] > 1:
                result += 1

    dump_matrix(matrix)

    return result

def solve(file_input: str = './input.txt'):
    result = int()
    array = list()
    x1, x2, y1, y2 = int(), int(), int(), int()
    all_maxes = list()

    with open(file_input, 'r') as fd:
        for line in fd:
            v1, v2 = line.strip().split("->")
            x1, y1 = v1.split(",")
            x2, y2 = v2.split(",")
            all_maxes.append(max(max(x1, x2), max(y1, y2)))
            array.append((int(x1),int(y1),int(x2),int(y2)))

    matrix_size = int(max(all_maxes)) + 1

    result = venture_part1(array, matrix_size)
    print("Result Part 1: {}".format(result))
    result = venture_part2(array, matrix_size)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    if len(sys.argv) == 1:
        solve()
    else:
        solve(sys.argv[1])
