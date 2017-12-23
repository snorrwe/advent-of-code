def tokenize(inp):
    result = []
    for line in inp:
        instr, register, amount = line.split(' ')
        result.append((instr, register, amount))
    return result


def run_program(tokens, registers):
    ptr = 0
    part1 = 0
    while ptr < len(tokens):
        instr, reg, value = tokens[ptr]
        try:
            value = int(value)
        except ValueError:
            value = registers[value]
        if instr == 'set':
            registers[reg] = value
        elif instr == 'sub':
            registers[reg] -= value
        elif instr == 'mul':
            registers[reg] *= value
            part1 += 1
        elif instr == 'jnz':
            jump = False
            try:
                jump = registers[reg]
            except KeyError:
                jump = reg
            if jump:
                ptr += value - 1
        ptr += 1
        print(registers)
    return (part1, registers)


def part1(inp):
    return None
    tokens = tokenize(inp)
    registers = {chr(ord('a') + i): 0 for i in range(8)}
    return run_program(tokens, registers)[0]


def part2(inp):
    tokens = tokenize(inp)
    registers = {chr(ord('a') + i): 0 for i in range(8)}
    registers['a'] = 1
    return run_program(tokens, registers)[1]['h']


def solve(inp):
    return(part1(inp), part2(inp))


def main():
    with open('input.txt', 'r') as f:
        print(solve(f.read().split('\n')))

if __name__ == '__main__':
    main()
