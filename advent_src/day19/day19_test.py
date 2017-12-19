import unittest
import pytest
from .day19 import solve

TEST = """     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ """.split('\n')


class Part1(unittest.TestCase):

    def test_simple(self):
        actual = solve(TEST)[0]
        self.assertEqual(actual, "ABCDEF")


class Part2(unittest.TestCase):

    def test_simple(self):
        actual = solve(TEST)[1]
        self.assertEqual(actual, 38)

if __name__ == '__main__':
    pytest.main()
