"""
Advent of Code 2020: Day 2 Password Philosophy

https://adventofcode.com/2020/day/2
"""
import sys

from typing import List, Union
from dataclasses import dataclass

@dataclass
class Policy:
    min_allowed: int
    max_allowed: int
    char: str

@dataclass
class Password:
    value: str
    policy: Policy

    def valid_password_part1(self) -> bool:
        is_valid = False

        if self.policy.char not in self.value:
            return False

        for c in list(self.value):
            if c == self.policy.char:
                    if self.policy.min_allowed <= self.value.count(c) \
                            <= self.policy.max_allowed:
                        is_valid = True
                    else:
                        is_valid = False

        return is_valid

    def valid_password_part2(self) -> bool:
        if self.policy.char not in self.value:
            return False

        if bool(self.value[self.policy.min_allowed-1] == self.policy.char) ^ \
                bool(self.value[self.policy.max_allowed-1] == self.policy.char):
            return True

        return False

def valid_passwords_part1(array):
    """
    Part 1

    Valid password with policy min and max number of characters
    """
    count = 0
    for p in array:
        if p.valid_password_part1():
            count+= 1

    return count


def valid_passwords_part2(array):
    """
    Part 2

    Valid password with policy position index matching only once
    """
    count = 0
    for p in array:
        if p.valid_password_part2():
            count+= 1

    return count

def solve(file_input: str):
    result = int()
    array = list()

    with open(file_input, 'r') as fd:
        for line in fd.readlines():
            min_max_policy, password = line.split(':')
            password = password.replace(' ','').strip()
            min_max, char = min_max_policy.split(' ')
            min_p, max_p = min_max.split('-')
            policy = Policy(int(min_p), int(max_p), char)
            p = Password(password, policy)
            array.append(p)

    result = valid_passwords_part1(array)
    print("Result Part 1: {}".format(result))
    result = valid_passwords_part2(array)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    solve(sys.argv[1])
