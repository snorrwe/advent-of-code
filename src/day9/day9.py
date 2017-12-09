#!/usr/bin/python


def solve(input):
    group_level = 0
    score = 0
    cancel = False
    garbage = False
    dumped_characters = 0
    for i in input:
        if cancel:
            cancel = False
            continue
        if not garbage:
            if i == '{':
                group_level += 1
                score += group_level
            elif i == '}':
                group_level -= 1
            elif i == '<':
                garbage = True
        else:
            if i == '>':
                garbage = False
            elif i == '!':
                cancel = True
            else:
                dumped_characters += 1
    return (score, dumped_characters)


def main():
    with open('input.txt', 'r') as f:
        print(solve(f.read()))

if __name__ == '__main__':
    main()
