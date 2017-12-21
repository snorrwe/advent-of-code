import unittest
import pytest
from .day21 import run, START_PATTERN


TEST_RULES = """../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#""".split('\n')


class Run_Tests(unittest.TestCase):

    def test_simple(self):
        actual = run(TEST_RULES, 2)
        self.assertEqual(actual, 12)

    def test_actual(self):
        with open("input.txt", 'r') as f:
            actual = run(f.read().split('\n'), 5)
            self.assertEqual(actual, 208)


if __name__ == '__main__':
    pytest.main()
