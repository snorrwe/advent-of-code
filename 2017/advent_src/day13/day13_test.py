import unittest
import pytest
from collections import defaultdict
from .day13 import solve, check_caught, init, tick


def part1(input):
    return solve(input)[0]


def part2(input):
    return solve(input)[1]

TEST_INPUT = """0: 3
1: 2
4: 4
6: 4"""


class Part1(unittest.TestCase):

    def test_simple(self):
        actual = part1(TEST_INPUT.split('\n'))
        self.assertEqual(actual, 24)

    # def test_actual(self):
    #     with open('input.txt', 'r') as f:
    #         actual = part1(f)
    #         self.assertEqual(actual, 2264)


class Part2(unittest.TestCase):

    def test_simple(self):
        actual = part2(TEST_INPUT.split('\n'))
        self.assertEqual(actual, 10)


class CheckCaught(unittest.TestCase):

    def test_simple(self):
        actual = check_caught(3, 0)
        self.assertTrue(actual)
        actual = check_caught(3, 1)
        self.assertFalse(actual)
        actual = check_caught(3, 2)
        self.assertFalse(actual)
        actual = check_caught(3, 3)
        self.assertFalse(actual)
        actual = check_caught(3, 4)
        self.assertTrue(actual)
        actual = check_caught(3, 5)
        self.assertFalse(actual)
        actual = check_caught(3, 6)
        self.assertFalse(actual)


if __name__ == '__main__':
    pytest.main()
