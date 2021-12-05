#!/usr/bin/python


def solve(input):
    jumps = [int(i) for i in input.split('\n')]
    ptr = 0
    steps = 0
    while -1 < ptr < len(jumps):
        jump = jumps[ptr]
        jumps[ptr] += 1
        ptr += jump
        steps += 1
    print(steps)


def main():
    solve(
        """0
3
0
1
-3""")
    with open('input.txt', 'r') as file:
        input = file.read()
        solve(input)

if __name__ == '__main__':
    main()
