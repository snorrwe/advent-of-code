from dataclasses import dataclass
from argparse import ArgumentParser
import re
import sys

import numpy as np
from z3 import *

parser = ArgumentParser()
parser.add_argument("file", default="input.txt")

args = parser.parse_args()


@dataclass()
class Row:
    wiring: list[list[int]]
    joltage: list[int]


def parse(f) -> list[Row]:
    rx = re.compile(r"^\[(.*)\]\s?((\([0-9,]+\)\s?)+)\s?(\{[0-9,]+\}\s?)+$")
    number_groups = re.compile(r"\((\d+,?)+\)")

    def _parse(f):
        for line in f:
            caps = rx.findall(line)
            if not caps:
                continue
            caps = caps[0]
            wirings = []
            for c in number_groups.finditer(line):
                nums = c.group(0)
                nums = nums.strip("() \n")
                wirings.append([int(x) for x in nums.split(",")])

            joltage = [int(x) for x in caps[3].strip("{}  \n").split(",")]

            yield (Row(wirings, joltage))

    return list(_parse(f))


if args.file == "-":
    input = parse(sys.stdin)
else:
    with open(args.file) as f:
        input = parse(f)


def solve_row(row: Row):
    m = np.zeros((len(row.joltage), len(row.wiring)))

    for x, w in enumerate(row.wiring):
        for y in w:
            m[y, x] = 1

    vars = [Int(f"x_{i}") for i in range(m.shape[1])]
    s = Optimize()
    for v in vars:
        s.add(v >= 0)
    for y, c in enumerate(row.joltage):
        v = []
        for x in range(m.shape[1]):
            if m[y, x]:
                v.append(vars[x])
        s.add(sum(v) == c)
    s.minimize(sum(vars))
    s.check()
    m = s.model()

    res = sum(m.eval(v).as_long() for v in vars)
    return res


print(sum(solve_row(r) for r in input))
