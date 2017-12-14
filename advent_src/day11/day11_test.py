import pytest
import unittest
from .day11 import solve


def part1(input):
    return solve(input)[0]


class Part1(unittest.TestCase):

    def test_simple_same_directions(self):
        actual = part1('ne,ne,ne')
        self.assertEqual(actual, 3)

    def test_simple_backtrack(self):
        actual = part1('ne,ne,sw,sw')
        self.assertEqual(actual, 0)

    def test_simple_move_around(self):
        actual = part1('ne,ne,s,s')
        self.assertEqual(actual, 2)

    def test_simple_move_around_2(self):
        actual = part1('se,sw,se,sw,sw')
        self.assertEqual(actual, 3)

if __name__ == '__main__':
    pytest.main()
