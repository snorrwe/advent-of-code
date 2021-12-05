#!/usr/bin/python
import re

TEST = """pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"""


def solve(input):
    tower = []
    for line in input:
        node = line.split("-> ")
        name = re.match(r'^([a-z]+) ', node[0])
        children = node[1].split(", ") if len(node) == 2 else []
        for child in children:
            tower.append((name.group(1), child.replace('\n', '')))
    nodes = set([i[0] for i in tower])
    for node in nodes:
        found = False
        for i in tower:
            if node == i[1]:
                found = True
        if not found:
            return node
    return None


def main():
    with open('input.txt', 'r') as input:
        result_t_1 = solve(TEST.split('\n'))
        result_actual_1 = solve(input)
        print(result_t_1)
        print(result_actual_1)

if __name__ == '__main__':
    main()
