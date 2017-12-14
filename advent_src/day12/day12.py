#!/usr/bin/python
import re
from collections import defaultdict


def process_group(id, connections, cnt):
    if id in connections:
        cnt += 1 + len(connections[id])
        todo = connections.pop(id)
        while todo:
            current = todo.pop()
            remaining = [i for i in connections.pop(current, [])]
            cnt += len(remaining)
            todo.extend(remaining)
    return cnt, connections


def solve(input):
    connections = defaultdict(lambda: [])
    for line in input:
        m = re.match(r'^(\d+)( <-> (((, )?\d+)+))?', line)
        line.split
        id = m.group(1)
        connections[id]
        if m.lastindex > 1:
            for i in m.group(3).split(','):
                if i:
                    connections[id].append(i.replace(' ', ''))
    groups = 1
    (cnt, connections) = process_group('0', connections, 0)
    while connections:
        groups += 1
        if groups > 10:
            print(connections)
            break
        n = list(connections.keys())[0]
        (cnt, connections) = process_group(n, connections, cnt)
    return (cnt, groups)


def main():
    with open('input.txt', 'r') as f:
        print(solve(f))

if __name__ == '__main__':
    main()
