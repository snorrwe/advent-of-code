#!/usr/bin/env python3
import sys

if len(sys.argv) > 1:
    with open(sys.argv[1]) as f:
        input = f.read()
else:
    input = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""

total = 0
for line in input.splitlines():
    first = 0
    last = 0
    for c in line:
        try:
            c = int(c)
            if not first:
                first = c
            last = c
        except:
            pass
    total += 10 * first + last

print(total)

digits = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
]

if len(sys.argv) > 1:
    with open(sys.argv[1]) as f:
        input = f.read()
else:
    input = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""



total = 0
for line in input.splitlines():
    first = 0
    last = 0
    for i, c in enumerate(line):
        try:
            c = int(c)
            if not first:
                first = c
            last = c
        except:
            for j, digit in enumerate(digits):
                if line[i:].startswith(digit):
                    c = j
                    if not first:
                        first = c
                    last = c
                    break
    total += 10 * first + last

print(total)
