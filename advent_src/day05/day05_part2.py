#!/usr/bin/python


def solve(input):
    jumps = [int(i) for i in input.split('\n')]
    ptr = 0
    steps = 0
    while -1 < ptr < len(jumps):
        jump = jumps[ptr]
        jumps[ptr] += 1 if jumps[ptr] < 3 else -1
        ptr += jump
        steps += 1
    print(steps)

TEST_INPUT = """0
3
0
1
-3"""


def main():
    solve(TEST_INPUT)  # 10
    with open('input.txt', 'r') as file:
        input = file.read()
        solve(input)  # 25347697

if __name__ == '__main__':
    main()
