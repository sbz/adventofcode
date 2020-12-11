"""
Advent of Code 2020: Day 4 Passport Processing

https://adventofcode.com/2020/day/4
"""
import re
import sys

from typing import List, Union
from dataclasses import dataclass

@dataclass
class Passport:
    fields: dict

    def is_valid_part1(self) -> bool:
        for mandatory in ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']:
            if mandatory not in self.fields:
                return False

        return True

    def is_valid_part2(self) -> bool:
        if not self.is_valid_part1():
            return False

        if not valid_year(self.fields['byr'], 1920, 2002) or \
                not valid_year(self.fields['iyr'], 2010, 2020) or \
                not valid_year(self.fields['eyr'], 2020, 2030):
            return False

        if not valid_height(self.fields['hgt']):
            return False

        if not valid_hair(self.fields['hcl']):
            return False

        if not valid_eye(self.fields['ecl']):
            return False

        if not valid_pid(self.fields['pid']):
            return False

        return True

def valid_year(year: str, min_y: int, max_y: int) -> bool:
    if len(year) != 4:
        return False

    if not min_y <= int(year) <= max_y:
        return False

    return True

def valid_height(hgt: str) -> bool:
    if 'cm' in hgt:
        if not 150 <= int(hgt.replace('cm','')) <= 193:
            return False
    elif 'in' in hgt:
        if not 59 <= int(hgt.replace('in', '')) <= 76:
            return False
    else:
        return False

    return True

def valid_hair(hcl: str) -> bool:
    pattern = re.compile(r'^#(\d|[a-f]){6}$')
    if not re.match(pattern, hcl):
        return False

    return True

def valid_eye(ecl: str) -> bool:
    pattern = re.compile(r'(amb|blu|brn|gry|grn|hzl|oth)')
    if not re.match(pattern, ecl):
        return False

    return True

def valid_pid(pid: str) -> bool:
    return len(pid) == 9

def passport_part1(passports: List[Passport]) -> bool:
    """
    Part 1, count valid passports
    """
    count = 0

    for p in passports:
        if p.is_valid_part1():
            count += 1

    return count


def passport_part2(passports: List[Passport]) -> int:
    """
    Part 2, count present and valid passports
    """
    count = 0

    for p in passports:
        if p.is_valid_part2():
            count += 1

    return count

def solve(file_input: str):
    result = int()
    array =  list()
    fields = dict()

    with open(file_input, 'r') as fd:
        lines = [l.strip() for l in fd.readlines()]
        for line in lines:
            if len(line) != 0:
                keys_values = line.split(' ')
                for key_value in keys_values:
                    if ':' in key_value:
                        key, value = key_value.split(':')
                        fields[key] = value
                p = Passport(fields)
                if p not in array:
                    array.append(Passport(fields))
            elif line == '':
                fields = dict()
                continue

    result = passport_part1(array)
    print("Result Part 1: {}".format(result))
    result = passport_part2(array)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    solve(sys.argv[1])
