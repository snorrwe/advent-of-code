BINARY_FORMAT = "{0:b}"


def binary_match(n, m):
    n = BINARY_FORMAT.format(n)[-16:]
    m = BINARY_FORMAT.format(m)[-16:]
    return n == m


def lcg(seed, a, m=2147483647):
    while 1:
        seed = (seed*a) % m
        yield seed


def lcg_pair(a, factor_a, b, factor_b, m=2147483647):
    a, b = lcg(a, factor_a, m), lcg(b, factor_b, m)
    while 1:
        yield (next(a), next(b))


def part1(initial, length=int(40e6)):
    print("Part1 starting")
    result = 0
    gen = lcg_pair(initial[0], 16807, initial[1], 48271)
    for _ in range(length):
        n, m = next(gen)
        if binary_match(n, m):
            result += 1
    print("Part1 done")
    return result


def part2(initial, length=int(5e6)):
    print("Part2 starting")
    result = 0
    processed = 0
    a, b = lcg(initial[0], 16807), lcg(initial[1], 48271)
    while processed < length:
        n, m = next(a), next(b)
        while n % 4 != 0:
            n = next(a)
        while m % 8 != 0:
            m = next(b)
        processed += 1
        if binary_match(n, m):
            result += 1
    print("Part2 done")
    return result


def solve(initial):
    return (part1(initial), part2(initial))


def main():
    print(solve((883, 879)))

if __name__ == '__main__':
    main()
