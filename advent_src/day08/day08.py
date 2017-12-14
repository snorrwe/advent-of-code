#!/usr/bin/python
from collections import defaultdict
import re
import json

TEST = """b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"""


def evaluate_condition(registers, target, condition, value):
    target = registers[target]
    return {
        '>': target > value,
        '>=': target >= value,
        '<': target < value,
        '<=': target <= value,
        '==': target == value,
        '!=': target != value,
    }[condition]


def action(registers, target, command, value):
    registers[target] += value if command == 'inc' else -value


def solve(input):
    registers = defaultdict(lambda: 0)
    all_time_max = 0
    for line in input:
        m = re.match(
            r"^([a-z]+) ([a-z]{3}) (-?\d+) if ([a-z]+) (.+) (-?\d+)$", line)
        target = (m.group(1), m.group(2), int(m.group(3)))
        condition = (m.group(4), m.group(5), int(m.group(6)))
        if(evaluate_condition(registers, *condition)):
            action(registers, *target)
            if(registers[target[0]] > all_time_max):
                all_time_max = registers[target[0]]
    print("part1:", max(registers.items(), key=lambda i: i[1]))
    print("part2:", all_time_max)


def main():
    solve(TEST.split("\n"))
    with open('input.txt', 'r') as f:
        solve(f)

if __name__ == '__main__':
    main()
