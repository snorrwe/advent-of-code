import unittest
import pytest
from .day19 import part1

TEST = """     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ """.split('\n')


class Part1(unittest.TestCase):

    def test_simple(self):
        actual = part1(TEST)
        self.assertEqual(actual, "ABCDEF")

if __name__ == '__main__':
    pytest.main()
