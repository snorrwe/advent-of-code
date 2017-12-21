import unittest
import pytest
from .day20 import part1, part2

TEST_INPUT_1 = """p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>""".split('\n')

TEST_INPUT_2 = """p=<0,0,0> v=<2,0,0> a=<-1,0,0>
p=<0,0,0> v=<1,0,0> a=<1,0,0>""".split('\n')

TEST_INPUT_3 = """p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>    
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>""".split('\n')

TEST_INPUT_4 = """p=<-7,0,0>, v=<3,0,0>, a=<1,0,0>    
p=<-4,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>""".split('\n')


class Part1(unittest.TestCase):

    def test_simple_1(self):
        actual = part1(TEST_INPUT_1)
        self.assertEqual(actual, 0)

    def test_simple_2(self):
        actual = part1(TEST_INPUT_2)
        self.assertEqual(actual, 1)

    def test_actual(self):
        with open("input.txt", 'r') as f:
            actual = part1(f.readlines())
            self.assertEqual(actual, 243)


class Part2(unittest.TestCase):

    def test_simple_1(self):
        actual = part2(TEST_INPUT_3)
        self.assertEqual(actual, 1)

    def test_simple_2(self):
        actual = part2(TEST_INPUT_4)
        self.assertEqual(actual, 1)


if __name__ == '__main__':
    pytest.main()
