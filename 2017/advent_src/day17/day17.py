def spin(steps, spins):
    buff = [0]
    pos = 0
    for i in range(1, spins+1):
        pos = (pos + steps) % len(buff)
        pos += 1
        buff.insert(pos, i)
    return (buff, pos)


def part1(steps, length=2017):
    buff, pos = spin(steps, length)
    return buff[pos+1]


def part2(steps, length=int(50e6)):
    result = None
    pos = 0
    for i in range(1, length+1):
        pos = (pos + steps) % i
        pos += 1
        if pos == 1:
            result = i
    return result


def solve(steps):
    return (part1(steps), part2(steps))


def main():
    print(solve(303))

if __name__ == '__main__':
    main()
