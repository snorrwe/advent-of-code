import enum

Direction = enum.Enum('Direction', 'n ne se s sw nw')


class Hex(object):
    """Represent a hex on the grid"""

    def __init__(self, q, r, s):
        assert(q + r + s == 0)
        self.q, self.r, self.s = q, r, s

    def __eq__(self, hex):
        return self.q == hex.q and self.r == hex.r and self.s == hex.s

    def __ne__(self, hex):
        return not (self == hex)

    def __add__(self, hex):
        return Hex(self.q + hex.q, self.r + hex.r, self.s + hex.s)

    def __sub__(self, hex):
        return Hex(self.q - hex.q, self.r - hex.r, self.s - hex.s)

    def __str__(self):
        return "[%s, %s, %s]" % (self.q, self.r, self.s)

    def length(self):
        return (abs(self.q) + abs(self.r) + abs(self.s)) // 2

    def distance(self, hex):
        return (self - hex).length()

    def neighbour(self, direction):
        assert(direction in Direction)
        hex_directions = {
            Direction.n: Hex(1, 0, -1),
            Direction.ne: Hex(1, -1, 0),
            Direction.se: Hex(0, -1, 1),
            Direction.s: Hex(-1, 0, 1),
            Direction.sw: Hex(-1, 1, 0),
            Direction.nw: Hex(0, 1, -1)
        }
        return self + hex_directions[direction]


def solve(input):
    start = Hex(0, 0, 0)
    current = start
    part2 = 0
    for direction in input.split(','):
        current = current.neighbour(Direction[direction])
        d_start = current.distance(start)
        if d_start > part2:
            part2 = d_start
    part1 = start.distance(current)
    return (part1, part2)


def main():
    with open('input.txt', 'r') as f:
        print(solve(f.read()))

if __name__ == '__main__':
    main()
