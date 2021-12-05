import pytest
import unittest
from .day15 import part1, part2, binary_match


class Part1(unittest.TestCase):

    def test_simple(self):
        actual = part1((65, 8921), 5)
        self.assertEqual(actual, 1)


class Part2(unittest.TestCase):

    def test_simple(self):
        actual = part2((65, 8921), 1100)
        self.assertEqual(actual, 1)


class BinaryMatch(unittest.TestCase):

    def test_1(self):
        actual = binary_match(245556042, 1431495498)
        self.assertTrue(actual)

    def test_2(self):
        actual = binary_match(1092455, 430625591)
        self.assertFalse(actual)

    def test_can_handle_small_numbers(self):
        actual = binary_match(5, 3)
        self.assertFalse(actual)

if __name__ == '__main__':
    pytest.main()
