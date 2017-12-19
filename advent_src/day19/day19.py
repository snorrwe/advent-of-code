
class Vector(object):

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, v):
        return self.x == v.x and self.y == v.y

    def __ne__(self, v):
        return not(self == v)

    def __hash__(self):
        return hash((self.x, self.y))


class Node(object):

    def __init__(self, x, y, value=None):
        self._neighbours = {}
        self.pos = Vector(x, y)
        self.value = value

    def next(self, last_pos):
        x, y = self.pos.x, self.pos.y
        velocity = Vector(x - last_pos.x, y - last_pos.y)
        next_vector = Vector(velocity.x + x, velocity.y + y)
        if next_vector in self._neighbours:
            return self._neighbours[next_vector]
        for vec in self._neighbours:
            if vec != last_pos:
                return self._neighbours[vec]
        return None

    def add_neighbour(self, node):
        if node.pos not in self._neighbours:
            self._neighbours[node.pos] = node
        if self.pos not in node._neighbours:
            node.add_neighbour(self)

    def __str__(self):
        neighbours = "\n".join(["[%s, %s]" % (i.pos.x, i.pos.y)
                                for i in self._neighbours.values()])
        return "[{x}, {y}]\nNeighbours:\n{n}".format(x=self.pos.x,
                                                     y=self.pos.y,
                                                     n=neighbours)


def create_map(diagram):
    nodes = {}
    start = None
    for x, char in enumerate(diagram[0]):
        if char != ' ':
            pos = Vector(x, 0)
            start = Node(x, 0)
            nodes[pos] = start
    for y, line in enumerate(diagram[1:]):
        y += 1
        for x, char in enumerate(line):
            if char != ' ':
                v = None
                if char.isalpha():
                    v = char
                pos = Vector(x, y)
                node = Node(x, y, v)
                nodes[pos] = node
                for v in [Vector(pos.x - 1, pos.y), Vector(pos.x, pos.y - 1)]:
                    if v in nodes:
                        nodes[v].add_neighbour(node)
    return (start, nodes)


def solve(diagram):
    start, nodes = create_map(diagram)
    current = start
    last = Vector(start.pos.x, -1)
    part1 = ""
    part2 = 0
    while 1:
        part2 += 1
        current, last = current.next(last), current.pos
        if not current:
            return (part1, part2)
        if current.value:
            part1 += current.value


def main():
    with open("input.txt", 'r') as f:
        print(solve(f.read().split('\n')))

if __name__ == '__main__':
    main()
