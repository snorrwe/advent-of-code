import unittest
import pytest
from .day12 import solve


def part1(input):
    return solve(input)[0]


def part2(input):
    return solve(input)[1]


class Part1(unittest.TestCase):

    def test_finds_itself(self):
        actual = part1(['0'])
        self.assertEqual(actual, 1)

    def test_finds_direct_connections(self):
        actual = part1(['0 <-> 2, 3'])
        self.assertEqual(actual, 3)

    def test_finds_indirect_connections(self):
        actual = part1("""0 <-> 2, 3
2 <-> 4, 5
4 <-> 6""".split('\n'))
        self.assertEqual(actual, 6)


class Part2(unittest.TestCase):

    def test_finds_single_group(self):
        actual = part2("""0 <-> 2, 3
2 <-> 4, 5
4 <-> 6""".split('\n'))
        self.assertEqual(actual, 1)

    def test_finds_non_trivial_groups(self):
        actual = part2("""0 <-> 2, 3
1 <-> 4
7 <-> 8
2 <-> 4, 5
4 <-> 6""".split('\n'))
        self.assertEqual(actual, 3)

if __name__ == '__main__':
    pytest.main()
