def fuel(mod):
    return mod // 3 - 2


def part1(file):
    return sum((fuel(int(mod)) for mod in file))


def part2(file):
    def gen(mod):
        mod = fuel(mod)
        while mod > 0:
            yield mod
            mod = fuel(mod)

    return sum((sum(gen(int(mod))) for mod in file))


with open("input.txt") as f:
    print(part1(f))

with open("input.txt") as f:
    print(part2(f))
