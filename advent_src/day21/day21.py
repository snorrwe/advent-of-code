from collections import Counter
import math

START_PATTERN = """.#.
..#
###""".split('\n')


class Grid(object):

    def __init__(self, pattern):
        self.world = [[c for c in line
                       if c is '.' or c is '#']
                      for line in pattern]
        self.cols = len(self.world)
        for row in self.world:
            assert len(row) == self.cols

    def count(self):
        return Counter([c for l in self.world for c in l])

    def __empty_world(self):
        return [[None for j in range(self.cols)] for i in range(self.cols)]

    def rotate(self):
        result = self.__empty_world()
        for i in range(self.cols):
            for j in range(self.cols):
                result[i][j] = self.world[self.cols - j - 1][i]
        return Grid([''.join(l) for l in result])

    def flip(self):
        result = []
        for row in self.world:
            result.append(reversed(row))
        return Grid([''.join(l) for l in result])

    def split(self):
        result = []
        split_range = 2 if self.cols % 2 == 0 else 3
        for i in range(0, self.cols, split_range):
            for j in range(0, self.cols, split_range):
                lines = [
                    ''.join([self.world[j + y][i + x]
                             for x in range(split_range)])
                    for y in range(split_range)
                ]
                result.append(Grid(lines))
        return result

    def __eq__(self, grid):
        for _ in range(4):
            if self.world == grid.world or self.world == grid.flip().world:
                return True
            else:
                grid = grid.rotate()
        return self.world == grid.flip().world

    def __hash__(self):
        return hash(''.join([''.join(l) for l in self.world]))

    def to_pattern(self):
        return [''.join(l) for l in self.world]

    def __str__(self):
        return '\n'.join(self.to_pattern())


def part1(rule_book, iterations=5):
    rules = {}
    for line in rule_book:
        grids = line.split(" => ")
        key = Grid(grids[0].split('/'))
        value = Grid(grids[1].split('/'))
        rules[key] = value
    current_grid = Grid(START_PATTERN)
    for _ in range(iterations):
        small_grids = current_grid.split()
        grids = []
        for g in small_grids:
            change_to = None
            for key, value in rules.items():
                if key == g:
                    change_to = value
                    break
            assert change_to, 'No rule found for pattern:\n%s' % g
            grids.append(change_to)
        assert len(small_grids) == len(grids)
        lines = []
        grid_per_col = int(math.sqrt(len(grids)))
        for i in range(0, len(grids), grid_per_col):
            new_lines = [[] for _ in range(grids[0].cols)]
            for g in grids[i: i+grid_per_col]:
                for j, v in enumerate(g.world, start=0):
                    new_lines[j] += v
            lines += new_lines
        current_grid = Grid([''.join(l) for l in lines])
    return sum([g.count()['#'] for g in grids])  # == 208?


def solve(inp):
    return (part1(inp), 0)


def main():
    with open("input.txt", 'r') as f:
        print(solve(f.readlines()))

if __name__ == '__main__':
    main()
