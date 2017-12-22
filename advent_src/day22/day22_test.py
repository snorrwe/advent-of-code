import unittest
import pytest
from .day22 import part1

TEST_INPUT = [
    """..#
#..
...""".split('\n')
]


class Part1(unittest.TestCase):

    def test_simple(self):
        actual = part1(TEST_INPUT[0], 7)
        self.assertEqual(actual, 5)

    def test_simple_k_70(self):
        actual = part1(TEST_INPUT[0], 70)
        self.assertEqual(actual, 41)

    def test_simple_large_k_10k(self):
        actual = part1(TEST_INPUT[0], int(1e4))
        self.assertEqual(actual, 5587)

if __name__ == '__main__':
    pytest.main()
