#!/usr/bin/python
import pytest
import unittest
from .day09 import solve as s


def solve1(input):
    return s(input)[0]


def solve2(input):
    return s(input)[1]


class Part1Test(unittest.TestCase):

    def test_finds_outer_group(self):
        actual = solve1(r'{}')
        self.assertEqual(actual, 1)

    def test_finds_inner_groups(self):
        actual = solve1(r'{{{}}}')
        self.assertEqual(actual, 1 + 2 + 3)
        actual = solve1(r'{{},{}}')
        self.assertEqual(actual, 1 + 2 + 2)

    def test_ignores_garbage(self):
        actual = solve1(r'{<a>,<a>,<a>,<a>}')
        self.assertEqual(actual, 1)
        actual = solve1(r'{{<ab>},{<ab>},{<ab>},{<ab>}}')
        self.assertEqual(actual, 1 + 2 + 2 + 2 + 2)
        actual = solve1(r'{{<{}>},{<{}>},{<{}>},{<{}>}}')
        self.assertEqual(actual, 1 + 2 + 2 + 2 + 2)

    def test_can_cancel_in_garbage(self):
        actual = solve1(r'{<!>{}>}')
        self.assertEqual(actual, 1)

    def test_ignores_cancel_in_well_formatter(self):
        actual = solve1(r'{!<{}>}')
        self.assertEqual(actual, 1)


class Part2Test(unittest.TestCase):

    def test_ignores_begin_end(self):
        actual = solve2(r'{<aa>}')
        self.assertEqual(actual, 2)

    def test_counts_characters(self):
        actual = solve2(r'<random characters>')
        self.assertEqual(actual, 17)

    def test_ignores_cancelled(self):
        actual = solve2(r'{<!aa>}')
        self.assertEqual(actual, 1)


if __name__ == '__main__':
    pytest.main()
