import unittest
import pytest
from .day14 import part1, part2, unhexify, remove_zone, count_zones


class Part1(unittest.TestCase):

    def test_simple(self):
        actual = part1("flqrgnkx")
        self.assertEqual(actual, 8108)


class Part2(unittest.TestCase):

    def test_simple(self):
        disk = {
            '0;0': 1,
            '0;1': 1,
            '1;0': 1,
            '1;1': 1,
            '0;5': 1,
            '0;6': 1
        }
        actual = count_zones(disk)
        self.assertEqual(actual, 2)

    def test_example(self):
        actual = part2("flqrgnkx")
        self.assertEqual(actual, 1242)


class Hex(unittest.TestCase):

    def test_can_convert_0(self):
        actual = unhexify('0')
        self.assertEqual(actual, '0000')

    def test_can_convert_1(self):
        actual = unhexify('1')
        self.assertEqual(actual, '0001')

    def test_can_convert_e(self):
        actual = unhexify('e')
        self.assertEqual(actual, '1110')


class RemoveZone(unittest.TestCase):

    def test_can_remove_single(self):
        disk = {'0;0': 1, '0,2': 1}
        remove_zone('0;0', disk)
        self.assertEqual(len(disk), 1)

    def test_can_remove_neighbours(self):
        disk = {
            '0;0': 1,
            '0;1': 1,
            '1;0': 1,
            '0;5': 1
        }
        remove_zone('0;0', disk)
        self.assertEqual(len(disk), 1)

    def test_can_remove_zone(self):
        disk = {
            '0;0': 1,
            '0;1': 1,
            '1;0': 1,
            '2;0': 1,
            '0;5': 1
        }
        remove_zone('0;0', disk)
        self.assertEqual(len(disk), 1)

if __name__ == '__main__':
    pytest.main()
