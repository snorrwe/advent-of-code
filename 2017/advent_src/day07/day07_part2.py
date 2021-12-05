#!/usr/bin/python
from collections import defaultdict, Counter
import json
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

    def __init__(self):
        self.children = defaultdict(lambda: TreeNode)
        self.siblings = set()
        self.weight = 0
        self.actual_weight = 0
        self.name = "Unknown"

    def calculate_weight(self):
        self.actual_weight = self.weight
        for _, child in self.children.items():
            self.actual_weight += child.calculate_weight()
        return self.actual_weight

    def to_dict(self):
        d = lambda: defaultdict(lambda: d())
        tree = {
            "children": {},
            "weight": self.weight,
            "actual_weight": self.actual_weight,
            "name": self.name
        }
        for child in self.children:
            tree["children"][child] = self.children[child].to_dict()
        return tree

    def __str__(self):
        return json.dumps(self.to_dict(), indent=4)


def find_tower(input):
    tower = []
    nodes = set()
    for line in input:
        node = line.split("-> ")
        name = re.match(r'^([a-z]+) \((\d+)\)', node[0])
        node_info = (name.group(1), int(name.group(2)))
        nodes.add(node_info)
        children = node[1].split(", ") if len(node) == 2 else []
        for child in children:
            tower.append((node_info, child.replace('\n', '')))
    return (tower, nodes)


def find_root(connections, nodes):
    for node in nodes:
        found = False
        for i in connections:
            if node[0] == i[1]:
                found = True
        if not found:
            return node


def build_node(children, weights, node):
    result = TreeNode()
    result.name = node
    result.weight = weights[node]
    for child in children[node]:
        node = build_node(children, weights, child)
        result.children[child] = node
    for name, child in result.children.items():
        child.siblings = set([result.children[i]
                              for i in result.children if i != name])
    return result


def build_tree(connections, nodes):
    children = defaultdict(lambda: [])
    weights = defaultdict(lambda: 0, **dict(nodes))
    for connection in connections:
        children[connection[0][0]].append(connection[1])
    root = find_root(connections, nodes)[0]
    tree = build_node(children, weights, root)
    tree.calculate_weight()
    return tree


def solve(input):
    connections, nodes = find_tower(input)
    tree = build_tree(connections, nodes)
    todo = [tree]
    while todo:
        current = todo.pop()
        weights = []
        for _, child in current.children.items():
            todo.append(child)
            weights.append(child.actual_weight)
        if len(set(weights)) > 1:
            count = Counter(weights).most_common()
            expected = count[0]
            actual = count[-1]
            diff = expected[0] - actual[0]
            affected = \
                [i.weight for _, i in current.children.items()
                 if i.actual_weight == actual[0]][0]
            print(affected + diff)


def main():
    with open('input.txt', 'r') as input:
        solve(TEST.split('\n'))
        solve(input)

if __name__ == '__main__':
    main()
