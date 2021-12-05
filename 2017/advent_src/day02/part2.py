#!usr/bin/python


def getValueByLine(row):
    for index, i in enumerate(row):
        for _, j in enumerate(row[index + 1:]):
            n, m = (i, j) if i >= j else (j, i)
            if n / m == n // m:
                return n // m
    raise ValueError()


def solve(input):
    checksum = 0
    for line in input:
        row = [int(i) for i in line.split(' ')]
        checksum += getValueByLine(row)
    return checksum


def main():
    with open("input.dat", 'r') as f:
        print(solve(f))


if __name__ == '__main__':
    main()
