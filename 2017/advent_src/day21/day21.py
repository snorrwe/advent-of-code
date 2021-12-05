import numpy as np
import math

START_PATTERN = """.#./..#/###"""


def grid(pattern):
    return np.array([[c == '#'for c in line]
                     for line in pattern.split('/')])


def init_rules(rule_book):
    rules = {}
    for line in rule_book:
        grids = line.split(" => ")
        key = grid(grids[0])
        flip_key = np.fliplr(key)
        value = grid(grids[1])
        for i in range(4):
            rules[key.tobytes()] = value
            rules[flip_key.tobytes()] = value
            key = np.rot90(key)
            flip_key = np.rot90(flip_key)
    return rules


def tick(grid, size, rules):
    split_size = 2 if size % 2 == 0 else 3
    new_size = size // split_size * (split_size + 1)
    new_grid = np.empty((new_size, new_size), dtype=bool)
    for i in range(0, size, split_size):
        for j in range(0, size, split_size):
            x = i//split_size*(split_size + 1)
            y = j//split_size*(split_size + 1)
            new_grid[x:x + split_size + 1,
                     y:y + split_size + 1] = \
                rules[grid[i:i+split_size, j:j+split_size].tobytes()]
    return new_grid, new_size


def run(rule_book, iterations):
    rules = init_rules(rule_book)
    current_grid = grid(START_PATTERN)
    current_size = 3
    for t in range(iterations):
        current_grid, current_size = tick(current_grid, current_size, rules)
    return sum(sum(current_grid))  # == 2480380?


def solve(inp):
    return (run(inp, 5), run(inp, 18))


def main():
    with open("input.txt", 'r') as f:
        print(solve(f.read().split('\n')))

if __name__ == '__main__':
    main()
