import re
from collections import defaultdict


class Vector3(object):

    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z
        self.length = abs(self.x) + abs(self.y) + abs(self.z)

    def __add__(self, v):
        return Vector3(self.x+v.x, self.y+v.y, self.z+v.y)

    def __sub__(self, v):
        return Vector3(self.x-v.x, self.y-v.y, self.z-v.y)

    def __mul__(self, x):
        return Vector3(self.x*x, self.y*x, self.z*x)

    def __lt__(self, v):
        return self.length < v.length

    def __le__(self, v):
        return self.length <= v.length

    def __gt__(self, v):
        return self.length > v.length

    def __ge__(self, v):
        return self.length >= v.length

    def distance(self, v):
        return (self + v).length

    def __str__(self):
        return "<%s, %s, %s>" % (self.x, self.y, self.z)


class Particle(object):

    def __init__(self, id, p, v, a):
        self.id = id
        self.p = p
        self.v = v
        self.a = a

    def update(self):
        self.v += self.a
        self.p += self.v

    def distance_from_origin(self, origin=Vector3(0, 0, 0)):
        return self.p.distance(origin)

    def pos_by_time(self, t=1e6):
        return self.p + (self.v * t) + (self.a * 0.5 * t * t)

    def dist_by_time(self, t=1e6):
        return self.pos_by_time(t).length

    def __lt__(self, p):
        if self.a.length != p.a.length:
            return self.a.length < p.a.length
        if self.v.length != p.v.length:
            return self.v.length < p.v.length
        return self.p.length < p.p.length

    def __str__(self):
        return """id: [%s]
| %s
| %s
| %s
""" % (self.id, self.p, self.v, self.a)


def build_space(raw_coordinates):

    world = {}
    for index, line in enumerate(raw_coordinates):
        vector_pattern = re.compile(r'<(-?\d+,-?\d+,-?\d+)>')
        coords = vector_pattern.findall(line)
        assert(len(coords) == 3)
        coords = [int(x) for i in coords for x in i.split(',')]
        world[index] = (Particle(index,
                                 Vector3(coords[0], coords[1], coords[2]),
                                 Vector3(coords[3], coords[4], coords[5]),
                                 Vector3(coords[6], coords[7], coords[8]),
                                 ))
    return world


def part1(raw_coordinates):
    world = build_space(raw_coordinates)
    result = min(world.values())
    print(result)
    return result.id


def solve(raw_coordinates):
    return (part1(raw_coordinates), 0)  # part1 <= 158


def main():
    with open("input.txt", 'r')as f:
        print(solve(f.readlines()))

if __name__ == '__main__':
    main()
