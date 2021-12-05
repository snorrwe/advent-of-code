import unittest
import pytest
from .day17 import part1, part2


class Part1(unittest.TestCase):

    def test_simple(self):
        actual = part1(3)
        self.assertEqual(actual, 638)


class Part2(unittest.TestCase):

    def test_simple(self):
        actual = part2(3, 6)
        self.assertEqual(actual, 5)

    def test_simple_larger(self):
        actual = part2(3, 2017)
        self.assertEqual(actual, 1226)

if __name__ == '__main__':
    pytest.main()
