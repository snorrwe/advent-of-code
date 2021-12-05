import pytest
import unittest
from .day16 import part1, part2


class Part1(unittest.TestCase):

    def test_can_spin(self):
        actual = part1(["s3"], 5)
        self.assertEqual(actual, "cdeab")

    def test_can_spin_1(self):
        actual = part1(["s1"], 5)
        self.assertEqual(actual, "eabcd")

    def test_can_exchange(self):
        actual = part1(['x3/4'], 5)
        self.assertEqual(actual, "abced")

    def test_can_partner(self):
        actual = part1(['pe/b'], 5)
        self.assertEqual(actual, "aecdb")

    def test_simple_dance(self):
        actual = part1(['s1', 'x3/4', 'pe/b'], 5)
        self.assertEqual(actual, "baedc")


class Part2(unittest.TestCase):

    def test_simple(self):
        actual = part2(['s1', 'x3/4', 'pe/b'], 5, 2)
        self.assertEqual(actual, "ceadb")

    def test_simple_large_epoch(self):
        actual = part2(['s1', 'x3/4', 'pe/b'], 5, 201)
        self.assertEqual(actual, "baedc")

if __name__ == '__main__':
    pytest.main()
