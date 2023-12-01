#!/usr/bin/env python3
import sys

if len(sys.argv) > 1:
    with open(sys.argv[1]) as f:
        input1 = f.read()
        input2 = input1
else:
    input1 = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""
    input2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""

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

total1 = 0
total2 = 0
for line in input1.splitlines():
    first1 = 0
    last1 = 0
    first2 = 0
    last2 = 0
    for i, c in enumerate(line):
        try:
            c = int(c)
            if not first1:
                first1 = c
                if not first2:
                    first2 = c
            last2 = last1 = c
        except:
            for j, digit in enumerate(digits):
                if line[i:].startswith(digit):
                    c = j
                    if not first2:
                        first2 = c
                    last2 = c
                    break
    total1 += 10 * first1 + last1
    total2 += 10 * first2 + last2

print(total1)
print(total2)
