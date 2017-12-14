#!/usr/bin/python
from collections import defaultdict
from copy import deepcopy


class Layer(object):

    def __init__(self):
        self.range = 0
        self.scanner = 0
        self.direction = 1

    def __str__(self):
        return "Layer [Range: [%s], Scanner: [%s], Direction: [%s]]" \
            % (self.range, self.scanner, self.direction)


def print_firewall(firewall, maxi):
    for i in range(maxi + 1):
        value = firewall[i] if firewall[i].range else "..."
        print(i, value)
    print("\n")


def init(input):
    firewall = defaultdict(lambda: Layer())
    maxi = 0
    for line in input:
        tokens = line.split(': ')
        key = int(tokens[0])
        value = int(tokens[1])
        firewall[key].range = value
        if key >= maxi:
            maxi = key
    return (firewall, maxi)


def tick(firewall, tick):
    tick += 1
    for key, layer in firewall.items():
        if layer.scanner == layer.range - 1:
            layer.direction = -1
        elif layer.scanner == 0:
            layer.direction = 1
        layer.scanner += layer.direction
    return (firewall, tick)


def part1(firewall, maxi):
    t = 0
    result = 0
    for _ in range(maxi + 1):
        firewall, t = tick(firewall, t)
        if firewall[t].scanner == 0:
            result += t * firewall[t].range
    return result


def check_caught(r, tick):
    return r > 0 and tick % (r * 2 - 2) == 0


def is_caught(firewall, holdout):
    for key, layer in firewall.items():
        if check_caught(layer.range, holdout+key):
            return True
    return False


def part2(firewall, maxi):
    holdout = 0
    caught = True
    while 1:
        if not is_caught(firewall, holdout):
            return holdout
        holdout += 1


def solve(input):
    firewall, maxi = init(input)
    result1 = part1(deepcopy(firewall), maxi)
    result2 = part2(firewall, maxi)
    return (result1, result2)  # Expected: (2264, 3875838)


def main():
    with open("input.txt", 'r') as f:
        print(solve(f))

if __name__ == '__main__':
    main()
