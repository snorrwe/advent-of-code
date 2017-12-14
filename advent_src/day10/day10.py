#!/usr/bin/python
import functools

INPUT = "230,1,2,221,97,252,168,169,57,99,0,254,181,255,235,167"


def knot(items, input, skip, pos):
    size = len(items)
    for i in input:
        affected = [items[x % size] for x in range(pos, pos + i)]
        affected = list(reversed(affected))
        for j in range(i):
            items[(pos + j) % size] = affected[j]
        pos = (pos + i + skip) % size
        skip += 1
    return (items, skip, pos)


def to_hex(n):
    assert(0 <= n <= 255)
    result = hex(n).replace("0x", "")
    if(len(result) < 2):
        if n < 16:
            result = "0%s" % result
        else:
            result = "%s0" % result
    return result


def xor(items):
    return functools.reduce(lambda x, y: x ^ y, items)


def part2(input, size=256, rounds=64):
    input = [ord(c) for c in input] + [17, 31, 73, 47, 23]
    items = [i for i in range(size)]
    skip, pos = (0, 0)
    for _ in range(rounds):
        items, skip, pos = knot(items, input, skip, pos)
    result = []
    for i in range(0, 256, 16):
        affected = items[i:i+16]
        assert(len(affected) == 16)
        output = xor(affected)
        result.append(output)
    result = "".join([to_hex(i) for i in result])
    return result


def part1(input, size=256, rounds=1):
    items = [i for i in range(size)]
    input = [int(i) for i in input.split(",")]
    skip, pos = (0, 0)
    for _ in range(rounds):
        items, skip, pos = knot(items, input, skip, pos)
    return items[0] * items[1]


def solve(input, size=256):
    return (part1(input, size), part2(input, size))


def main():
    print(solve(INPUT))

if __name__ == '__main__':
    main()
