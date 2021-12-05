from collections import defaultdict
from enum import Enum


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


def create_cluster(inp, clean=False, infected=True):
    cluster = {((i, j), infected if c == '#' else clean)
               for j, line in enumerate(inp)
               for i, c in enumerate(line)}
    return defaultdict(lambda: clean, cluster)


def part1(inp, bursts):
    cluster = create_cluster(inp)
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


State = Enum("State", "clean, weak, infected, flagged")


def part2(inp, bursts):
    cluster = create_cluster(inp, State.clean, State.infected)
    pos = (len(inp)//2,
           len(inp)//2)
    v = (0, -1)
    result = 0
    for t in range(bursts):
        if cluster[pos] == State.infected:
            v = turn_left(v)  # inverted because of the map
        elif cluster[pos] == State.clean:
            v = turn_right(v)  # inverted because of the map
        elif cluster[pos] == State.flagged:
            v = (-v[0], -v[1])
        elif cluster[pos] == State.weak:
            result += 1
        else:
            raise RuntimeError("Unknown State %s" % cluster[pos])
        cluster[pos] = {
            State.clean: State.weak,
            State.weak: State.infected,
            State.infected: State.flagged,
            State.flagged: State.clean
        }[cluster[pos]]
        pos = (pos[0] + v[0], pos[1] + v[1])
    return result


def solve(inp):
    return (part1(inp, 10000), part2(inp, 10000000))


def main():
    with open("input.txt", 'r') as f:
        print(solve(f.read().split('\n')))

if __name__ == '__main__':
    main()
