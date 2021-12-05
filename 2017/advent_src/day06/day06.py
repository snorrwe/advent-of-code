#!/usr/bin/python
import operator

TEST = [0, 2, 7, 0]
ACTUAL = [int(i) for i in
          "4 10 4 1 8 4 9 14 5 1 14 15 0 15 3 5".split(' ')]


def solve(items):
    seen = set()
    iterations = 0
    while 1:
        iterations += 1
        index, value = max(enumerate(items), key=lambda v: v[1])
        items[index] = 0
        while value:
            index += 1
            items[index % len(items)] += 1
            value -= 1

        hash = " ".join([str(i) for i in items])
        if hash in seen:
            return hash, iterations
        else:
            seen.add(hash)


def main():
    hash, result = solve(ACTUAL)
    print(hash, ": ", result)

if __name__ == '__main__':
    main()
