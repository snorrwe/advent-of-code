#!/usr/bin/python
from .knothash import knothash


def cached(fn):
    cache = {}
    def result(*args, **kwargs):
        if args in cache:
            return cache[args]
        r = fn(*args, **kwargs)
        cache[args] = r
        return r
    return result


def unhexify(hex):
    try:
        n = int(hex)
        result = ''
        for i in range(3, -1, -1):
            part = n // pow(2, i)
            n -= part * pow(2, i)
            result += str(part)
        return result
    except ValueError:
        return {
            'a': '1010',
            'b': '1011',
            'c': '1100',
            'd': '1101',
            'e': '1110',
            'f': '1111',
        }[hex]


@cached
def build_disk(input):
    disk = {}
    for i in range(128):
        seed = "%s-%s" % (input, i)
        hsh = knothash(seed)
        for j, n in enumerate([i for c in hsh for i in unhexify(c)]):
            if n == '1':
                disk["%s;%s" % (i, j)] = n
    return disk


def part1(input):
    return len(build_disk(input))


def neighbours(pos):
    x, y = pos.split(';')
    x, y = int(x), int(y)
    result = set()
    for i in range(-1, 2, 2):
        result.add("%s;%s" % (x, y + i))
        result.add("%s;%s" % (x + i, y))
    return result


def remove_zone(start, disk):
    todo = set([start])
    del disk[start]
    processed = set()
    while todo:
        current = todo.pop()
        if current not in processed:
            processed.add(current)
            for pos in [i for i in neighbours(current) if i not in processed]:
                try:
                    del disk[pos]
                    todo.add(pos)
                except KeyError:
                    pass


def count_zones(disk):
    disk = {**disk}
    cnt = 0
    while disk:
        cnt += 1
        zone_start = list(disk.keys())[0]
        remove_zone(zone_start, disk)
    return cnt


def part2(input):
    disk = build_disk(input)
    return count_zones(disk)


def solve(input):
    return (part1(input), part2(input))


def main():
    print(solve('ugkiagan'))

if __name__ == '__main__':
    main()
