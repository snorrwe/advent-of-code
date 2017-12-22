from collections import defaultdict


def turn_left(v):
    return {
        (1, 0): (0, 1),
        (0, 1): (-1, 0),
        (-1, 0): (0, -1),
        (0, -1): (1, 0)
    }[v]


def turn_right(v):
    return {
        (1, 0): (0, -1),
        (0, 1): (1, 0),
        (-1, 0): (0, 1),
        (0, -1): (-1, 0)
    }[v]


def part1(inp, bursts):
    cluster = {((i, j), c == '#')
               for j, line in enumerate(inp)
               for i, c in enumerate(line)}
    cluster = defaultdict(lambda: False, cluster)
    pos = (len(inp)//2,
           len(inp)//2)
    v = (0, -1)  # in the cluster's indexing y=-1 is "up"
    result = 0
    for t in range(bursts):
        if cluster[pos]:
            v = turn_left(v)
        else:
            v = turn_right(v)
            result += 1
        cluster[pos] = not cluster[pos]
        pos = (pos[0] + v[0], pos[1] + v[1])
    return result


def solve(inp):
    return (part1(inp, 10000), 0)


def main():
    with open("input.txt", 'r') as f:
        print(solve(f.read().split('\n')))

if __name__ == '__main__':
    main()
