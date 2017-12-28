import unittest
import pytest
from .day23 import part1, part2


class Part1(unittest.TestCase):

    def test_actual(self):
        with open('input.txt', 'r') as f:
            actual = part1(f.read().split('\n'))
        self.assertEqual(actual, 5929)


def test_part2():
    assert part2() == 907


if __name__ == '__main__':
    pytest.main()
