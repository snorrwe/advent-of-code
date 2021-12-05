import unittest
import pytest
from .day10 import part2, part1


def solve1(*args):
    return part1(*args)


def solve2(*args):
    return part2(*args)


class Day10Part1(unittest.TestCase):

    def test_simple(self):
        result = solve1("3,4,1,5", 5)
        self.assertEquals(result, 3 * 4)


class Day10Part2(unittest.TestCase):

    def test_length(self):
        result = solve2("1,2,3")
        self.assertEquals(len(result), 32)

    def test_empty(self):
        result = solve2("")
        self.assertEquals(result, "a2582a3a0e66e6e86e3812dcb672a272")

    def test_aoc(self):
        result = solve2("AoC 2017")
        self.assertEquals(result, "33efeb34ea91902bb2f59c9920caa6cd")

if __name__ == '__main__':
    pytests.main()
