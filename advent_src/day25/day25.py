#-*-coding:utf8;-*-
from collections import defaultdict

class State(object):

    def __init__(self, one, zero):
        self.one = one
        self.zero = zero
       
    def run(self, inp):
        if inp:
            return self.one
        return self.zero
        
TEST = {
'A': State((0, -1, 'B'), (1, 1, 'B')),
'B': State((1, 1, 'A'), (1, -1, 'A'))
}

def run(states, epochs, s='A'):
    pos = 0
    tape = defaultdict(lambda: 0)
    for t in range(epochs):
        value, d_pos, s = states[s].run(tape[pos])
        tape[pos] = value
        pos += d_pos
    return sum(tape.values())

assert run(TEST, 6) == 3
print('passes test')

ACTUAL = {
    'A': State((0, -1, 'F'), (1, 1, 'B')),
    'B': State((0, 1, 'D'), (0, 1, 'C')),
    'C': State((1, 1, 'E'), (1, -1, 'D')),
    'D': State((0, -1, 'D'), (0, -1, 'E')),
    'E': State((1, 1, 'C'), (0, 1, 'A')),
    'F': State((1, 1, 'A'), (1, -1, 'A'))
}

print(run(ACTUAL, 12794428))
