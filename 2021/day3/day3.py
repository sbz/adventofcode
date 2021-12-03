"""
Advent of Code 2021: Day 3 Binary Diagnostic

https://adventofcode.com/2021/day/3
"""
import sys

from collections import Counter
from typing import List, Union

def most_common_bits(array: List[str], index: int) -> int:
	all_bits = []
	for bits in array:
		all_bits.append(bits[index])

	return Counter(all_bits).most_common()[0][0]

def least_common_bits(array: List[str], index: int) -> int:
	all_bits = []
	for bits in array:
		all_bits.append(bits[index])

	return Counter(all_bits).most_common()[-1][0]

def bin_to_dec(array: List[str]):
	str_bin_value = "".join([str(x) for x in array])
	dec = int(str_bin_value, 2)
	return dec

def binary_part1(array: List[str]) -> int:
	gamma = []
	epsilon = []

	for i in range(len(array[0])):
		gamma.append(most_common_bits(array, i))
		epsilon.append(least_common_bits(array, i))

	return bin_to_dec(gamma) * bin_to_dec(epsilon)

def count_zeros(bits: List[str], index: int) -> int:
	count = 0
	all_bits = [int(b[index]) for b in bits]
	for b in all_bits:
		if b == 0:
			count += 1

	return count
	
def count_ones(bits: List[str], index: int) -> int:
	count = 0
	all_bits = [int(b[index]) for b in bits]
	for b in all_bits:
		if b == 1:
			count += 1

	return count

def oxygen_generate(bits: List[str], index: int) -> List[str]:
	nth_bit = 0
	while len(bits) > 1:
		new_bits = []
		nb_zeros = count_zeros(bits, nth_bit)
		nb_ones = count_ones(bits, nth_bit)
		selected = '0' if nb_zeros > nb_ones else '1'
		if nb_zeros == nb_ones: selected = '1'
		for b in bits:
			if b[nth_bit] == selected:
				new_bits.append(b)
		bits = new_bits
		nth_bit += 1

	return bits[0]

def co2_generate(bits: List[str], index: int) -> List[str]:
	nth_bit = 0
	while len(bits) > 1:
		new_bits = []
		nb_zeros = count_zeros(bits, nth_bit)
		nb_ones = count_ones(bits, nth_bit)
		selected = '0' if nb_zeros < nb_ones else '1'
		if nb_zeros == nb_ones: selected = '0'
		for b in bits:
			if b[nth_bit] == selected:
				new_bits.append(b)
		bits = new_bits
		nth_bit += 1

	return bits[0]

def binary_part2(array: List[int]) -> int:
	oxygen = []
	co2 = []

	oxygen = oxygen_generate(array, 0)
	co2 = co2_generate(array, 0)

	return bin_to_dec(oxygen) * bin_to_dec(co2)


def solve(file_input: str = './input.txt'):
    result = int()
    array = list()

    with open(file_input, 'r') as fd:
        for line in fd:
            value = line.strip()
            array.append(list(value))

    result = binary_part1(array)
    print("Result Part 1: {}".format(result))
    result = binary_part2(array)
    print("Result Part 2: {}".format(result))

if __name__ == '__main__':
    if len(sys.argv) == 1:
        solve()
    else:
        solve(sys.argv[1])
