"""
Advent of Code 2020: Day 5 Binary Boarding

https://adventofcode.com/2020/day/5
"""
import re
import sys

from typing import List, Union
from dataclasses import dataclass

@dataclass
class Range:
    lower: int
    upper: int

    def lower_half(self):
        self.lower = self.lower
        self.upper = self.lower + (self.upper - self.lower) // 2

    def upper_half(self):
        self.lower = self.lower + (self.upper - self.lower) // 2 + 1
        self.upper = self.upper

@dataclass
class Seat:
    row: Range
    col: Range
    seatid: int = -1

    def get_seatid(self) -> int:
        return min(self.row.lower, self.row.upper) * 8 + max(self.col.lower,
                                                             self.col.upper)

def decode_boarding_pass(item: str) -> int:
    row = Range(0, 127)
    col = Range(0, 7)
    seat = Seat(row, col)
    for c in item:
        if c == 'F':
            seat.row.lower_half()
        if c == 'B':
            seat.row.upper_half()
        if c == 'L':
            seat.col.lower_half()
        if c == 'R':
            seat.col.upper_half()

    return seat.get_seatid()


def seat_part1(array: List[str]) -> int:
    """
    Part 1

    Decode boarding pass row and column seat and find assigned seat id
    """
    list_seats_id = []

    for item in array:
        seat_id = decode_boarding_pass(item)
        list_seats_id.append(seat_id)

    return max(list_seats_id)

def seat_part2(array: List[str]) -> int:
    """
    Part 2

    Found your own missing seat id matching the given conditions
    """

    list_seats_id = [decode_boarding_pass(item) for item in array]
    list_seats_id.sort()
    all_seats_id = list(range(list_seats_id[0], list_seats_id[-1]+1))

    return list(set(all_seats_id) - set(list_seats_id))[0]


def solve(file_input: str):
    result = int()
    array =  list()

    with open(file_input, 'r') as fd:
        while True:
            board_pass = fd.readline().strip()
            if board_pass == '': break
            array.append(board_pass)

    result = seat_part1(array)
    print("Result Part 1: {}".format(result))
    result = seat_part2(array)
    print("Result Part 2: {}".format(result))


if __name__ == '__main__':
    solve(sys.argv[1])
