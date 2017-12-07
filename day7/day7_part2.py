#!/usr/bin/python
from collections import defaultdict
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


class TreeNode(object):

    def __init__(self, children=defaultdict(lambda: TreeNode)):
        self.children = children
        self.siblings = set()


def find_tower(input):
    tower = []
    for line in input:
        node = line.split("-> ")
        name = re.match(r'^([a-z]+) \((\d+)\)', node[0])
        children = node[1].split(", ") if len(node) == 2 else []
        for child in children:
            node = (name.group(1), int(name.group(2)))
            tower.append((node, child.replace('\n', '')))
    nodes = set([i[0] for i in tower])
    return (tower, nodes)


def find_root(connections, nodes):
    for node in nodes:
        found = False
        for i in connections:
            if node[0] == i[1]:
                found = True
        if not found:
            return node


def build_tree(connections, nodes):
    children = defaultdict(lambda: [])
    for connection in connections:
        children[connection[0]].append(connection[1])
    root = find_root(connections, nodes)


def solve(input):
    connections, nodes = find_tower(input)
    tree = build_tree(connections, nodes)


def main():
    with open('input.txt', 'r') as input:
        s = lambda input: print(solve(input))
        s(TEST.split('\n'))
        # s(input)

if __name__ == '__main__':
    main()
