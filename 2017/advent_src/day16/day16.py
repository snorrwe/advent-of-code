
def spin(programs, n):
    programs = programs[-n:] + programs[:-n]
    return programs


def exchange(programs, i, j):
    programs[i], programs[j] = programs[j], programs[i]
    return programs


def partner(programs, a, b):
    i = programs.index(a)
    j = programs.index(b)
    return exchange(programs, i, j)


def part1(input, number_of_programs=16, programs=None):
    if not programs:
        programs = [chr(ord('a') + i) for i in range(number_of_programs)]
    else:
        assert (len(programs) == number_of_programs)
    for i in input:
        programs = {
            's': lambda: spin(programs, int(i[1:])),
            'x': lambda: exchange(programs,
                                  *[int(j) for j in i[1:].split('/')]),
            'p': lambda: partner(programs, *[j for j in i[1:].split('/')])
        }[i[0]]()
        assert(len(programs) == number_of_programs)
    return ''.join(programs)


def find_period(input, number_of_programs, epochs):
    default = [chr(ord('a') + i) for i in range(number_of_programs)]
    split = lambda x: [i for i in x]
    indexes = lambda: [(programs.index(i)-default.index(i)) for i in programs]
    period = None
    cycle_start = None
    programs = split(part1(input, number_of_programs))
    for i in range(1, epochs):
        programs = split(part1(input, number_of_programs, programs))
        if indexes() == [0 for _ in range(number_of_programs)]:
            period = i + 1
            break
    return (period, programs)


def part2(input, number_of_programs=16, epochs=int(1e9)):
    period_info = find_period(input, number_of_programs, epochs)
    period, programs = period_info
    print(period_info)
    if period:
        for i in range(epochs % period):
            programs = [x for x in part1(input, number_of_programs, programs)]
    return ''.join(programs)


def solve(input, number_of_programs=16):
    return (part1(input, number_of_programs), part2(input, number_of_programs))


def main():
    with open('input.txt', 'r') as f:
        print(solve(f.read().split(',')))

if __name__ == '__main__':
    main()
