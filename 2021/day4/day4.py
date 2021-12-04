"""
Advent of Code 2021: Day 4 Giant Squid

https://adventofcode.com/2021/day/4
"""
import sys
import copy

from collections import OrderedDict
from dataclasses import dataclass
from pprint import pprint
from typing import List, Union, Tuple

@dataclass
class Board:
    matrix: List[List[int]]
    rows: int = 5
    cols: int = 5

    def create_marked(self):
        self.marked = [self.cols*[False] for _ in range(self.rows)]

def dump_board(board: Board):
    pprint(board.matrix, indent=2)
    if hasattr(board, 'marked'):
        pprint(board.marked, indent=2)

def has_win_row(board: Board) -> bool:
    for num in board.marked:
        if all(num) == True:
            return True

def has_win_col(board: Board) -> bool:
    colums = list(zip(*board.marked))
    for num in colums:
        if all(num) == True:
            return True

def has_win_board(board: Board) -> bool:
    return has_win_row(board) or has_win_col(board)

def mark_board(board: Board, num: int) -> Tuple[bool, int]:
    for row in range(board.rows):
        for col in range(board.cols):
            if board.matrix[row][col] == num:
                board.marked[row][col] = True
                if has_win_board(board):
                    return True, num

    return False, num

def sum_board(board: Board) -> int:
    total = 0

    for row in range(board.rows):
        for col in range(board.cols):
            if not board.marked[row][col]:
                total += board.matrix[row][col]

    return total


def giant_part1(boards: List[List[int]], bingo_nums: List[int]) -> int:
    result = 0
    success_board = -1

    [b.create_marked() for b in boards]

    while len(bingo_nums) > 1:
        draw = bingo_nums.pop(0)
        for nth_board, board in enumerate(boards):
            has_win, last_num = mark_board(board, draw)
            if has_win:
                success_board = nth_board + 1
                result = sum_board(board) * last_num
                return result

def giant_part2(boards: List[List[int]], bingo_nums: List[int]) -> int:
    result = {}
    success_board = -1
    winning_boards = OrderedDict()

    [b.create_marked() for b in boards]

    while len(bingo_nums) > 1:
        draw = bingo_nums.pop(0)
        for nth_board, board in enumerate(boards):
            has_win, last_num = mark_board(board, draw)
            if has_win:
                success_board = nth_board + 1
                winning_boards[success_board] = True
                if success_board not in result:
                    result[success_board] = sum_board(board) * last_num

    last_board_id = list(winning_boards.keys())[-1]
    result = result[last_board_id]

    return result

def solve(file_input: str = './input.txt'):
    result = int()
    array = list()
    boards = list()

    with open(file_input, 'r') as fd:
        bingo_nums = fd.readline().strip()
        array = [int(x) for x in bingo_nums.split(',')]
        bingo_board = []
        i = 0
        for line in fd:
            if line == '': 
                continue
            row = line.strip()
            if not row:
                continue
            bingo_board.append([int(x) for x in row.split()])
            i += 1
            if i == 5:
                boards.append(Board(bingo_board))
                bingo_board = []
                i = 0

    bingo_nums_part1 = copy.deepcopy(array)
    bingo_nums_part2 = copy.deepcopy(array)

    result = giant_part1(boards, bingo_nums_part1)
    print("Result Part 1: {}".format(result))
    result = giant_part2(boards, bingo_nums_part2)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    if len(sys.argv) == 1:
        solve()
    else:
        solve(sys.argv[1])
