from collections import defaultdict

TEST_INPUT = """0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10""".split('\n')

ACTUAL_INPUT = """24/14
30/24 
29/44
47/37
6/14
20/37
14/45
5/5
26/44
2/31
19/40
47/11
0/45
36/31
3/32
30/35
32/41
39/30
46/50
33/33
0/39
44/30
49/4
41/50
50/36
5/31
49/41
20/24
38/23
4/30
40/44
44/5
0/43
38/20
20/16
34/38
5/37
40/24
22/17
17/3
9/11
41/35
42/7
22/48
47/45
6/28
23/40
15/15
29/12
45/11
21/31
27/8
18/44
2/17
46/17
29/29
45/50""".split('\n')


def make_tree(inp):
    connections = defaultdict(list)
    nodes = set()
    for line in inp:
        x, y = line.split('/')
        x, y = int(x), int(y)
        n = (line, x, y)
        connections[x].append(n)
        connections[y].append(n)
        assert n not in nodes, "Oh noes"
        nodes.add(n)
    return connections


def walk(tree, root, direction, visited=None):
    visited = visited if visited else set()
    visited.add(root)
    sub_values = []
    children = {}
    for i in [r for r in tree[direction] if not visited or r not in visited]:
        v = set(visited)
        d = i[1] if i[1] != direction else i[2]
        result = walk(tree, i, d, v)
        sub_values.append(result[0])
        children['%s,%s,%s' % i] = result[1]
    value = sum(root[1:])
    part1 = value + (max([i for i in sub_values]) if sub_values else 0)
    return (part1, {'value': value, 'max': part1, 'children': children})


def part1(inp):
    tree = make_tree(inp)
    return walk(tree, (0, 0, 0), 0)


def longest(tree, p=False):
    results = []
    for k, v in tree['children'].items():
        r = longest(v)
        results.append((tree['value'] + r[0], r[1] + 1))
    if not results:
        return (tree['value'], 0)
    m = results[0]
    for i in results[1:]:
        if i[1] > m[1] or (i[1] == m[1] and i[0] > m[0]):
            m = i
    return m


def part2(tree):
    result = longest(tree, 1)
    return result[0]

r_test, tree_1 = part1(TEST_INPUT)
assert r_test == 31, "Doesn't pass part 1 test, result = [%s]" % r_test

r_test = part2(tree_1)
assert r_test == 19, "Doesn't pass part 2 test, result = [%s]" % r_test

result_1, tree = part1(ACTUAL_INPUT)
print(result_1)

result_2 = part2(tree)
print(result_2)
