#!/usr/bin/python
from collections import Counter


def can_rearrange_into(line):
    for word in line:
        for w in line:
            if word == w or len(word) != len(w):
                continue
            if Counter(word) == Counter(w):
                return True
    return False


def all_unique(line):
    return len(line) == len(set(line))


def main():
    with open('input.txt', 'r') as file:
        result1 = 0
        result2 = 0
        for line in file:
            line = line.replace('\n', '')
            tokens = line.split(' ')
            if all_unique(tokens):
                result1 += 1
                if not can_rearrange_into(tokens):
                    result2 += 1

        print('Part1: ', result1)  # 455
        print('Part2: ', result2)  # 186

if __name__ == '__main__':
    main()
