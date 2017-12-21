import unittest
import pytest
from .day21 import part1, Grid, START_PATTERN


TEST_RULES = """../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#""".split('\n')


class GridTests(unittest.TestCase):

    def test_can_create(self):
        grid = Grid(START_PATTERN)

    def test_can_count_ON(self):
        grid = Grid(START_PATTERN)
        actual = grid.count()
        self.assertEqual(actual['#'], 5)

    def test_can_rotate(self):
        grid1 = Grid(""".#.
..#
###""".split('\n')).rotate()
        grid2 = Grid("""#..
#.#
##.""".split('\n'))
        self.assertEqual(grid1.world, grid2.world)

    def test_equals_same(self):
        grid1 = Grid(""".#.
..#
###""".split('\n'))
        grid2 = Grid(""".#.
..#
###""".split('\n'))
        self.assertTrue(grid1 == grid2)

    def test_equals_rotated(self):
        grid1 = Grid(""".#.
..#
###""".split('\n'))
        grid2 = Grid("""#..
#.#
##.""".split('\n'))
        self.assertTrue(grid1 == grid2)

    def test_can_flip(self):
        grid1 = Grid(""".#.
..#
##.""".split('\n')).flip()
        grid2 = Grid(""".#.
#..
.##""".split('\n'))
        self.assertEqual(grid1.world, grid2.world)

    def test_equals_flipped(self):
        grid1 = Grid(""".#.
..#
##.""".split('\n'))
        grid2 = Grid(""".#.
#..
.##""".split('\n'))
        self.assertEqual(grid1, grid2)

    def test_can_split_3_by_3(self):
        grid = Grid(""".#.
..#
##.""".split('\n'))
        grids = grid.split()
        self.assertEqual(len(grids), 1)
        self.assertEqual(grids[0], grid)

    def test_can_split_4_by_4(self):
        grids = Grid(""".#..
..#.
##..
####""".split('\n')).split()
        self.assertEqual(len(grids), 4)
        self.assertEqual(grids[0].world, [['.', '#'], ['.', '.']])
        self.assertEqual(grids[-1].world, [['.', '.'], ['#', '#']])


class Part1(unittest.TestCase):

    def test_simple(self):
        actual = part1(TEST_RULES, 2)
        self.assertEqual(actual, 12)


if __name__ == '__main__':
    pytest.main()
