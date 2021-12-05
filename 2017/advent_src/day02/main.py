#!usr/bin/python


def solve(input):
    checksum = 0
    for line in input:
        row = [int(i) for i in line.split(' ')]
        mini = row[0]
        maxi = row[0]
        for item in row[1:]:
            if item < mini:
                mini = item
            elif item > maxi:
                maxi = item
        checksum += maxi - mini
    return checksum


def main():
    with open("input.dat", 'r') as f:
        print(solve(f))

if __name__ == '__main__':
    main()
