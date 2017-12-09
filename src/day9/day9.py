#!/usr/bin/python
import enum

State = enum.Enum('State', 'default garbage ignore')


def solve(input):
    score = 0
    dumped_characters = 0
    group_level = 0
    state = State.default
    for i in input:
        if state == State.ignore:
            state = State.garbage
            continue
        if state == State.default:
            if i == '{':
                group_level += 1
                score += group_level
            elif i == '}':
                group_level -= 1
            elif i == '<':
                state = State.garbage
        else:
            if i == '>':
                state = State.default
            elif i == '!':
                state = State.ignore
            else:
                dumped_characters += 1
    return (score, dumped_characters)


def main():
    with open('input.txt', 'r') as f:
        print(solve(f.read()))

if __name__ == '__main__':
    main()
