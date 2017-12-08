#!usr/bin/python


class Spiral(object):

    def __init__(self):
        self.world = {'0;0': 1}

    def find(self, n):
        current = 0
        x = 1
        y = 0
        radius = 1
        v = (0, 1)
        while current < n:
            s = 0
            for neighbour in self.__neighbours(x, y):
                s += neighbour[0]
                index = '%s;%s' % (neighbour[1], neighbour[2])
                self.world[index] = neighbour[0]

            current = s
            self.world['%s;%s' % (x, y)] = current

            r2 = radius * 2
            _x, _y = x + v[0], y + v[1]
            if radius <= self.distance(0, 0, _x, _y) <= r2:
                x, y = _x, _y
            else:
                v = (-v[1], v[0])
                if v == (0, 1):
                    x += 1
                    radius += 1
        return current

    def distance(self, x1, y1, x2, y2):
        return abs(x2 - x1) + abs(y2 - y1)

    def __neighbours(self, x, y):
        result = []
        for i in range(-1, 2):
            for j in range(-1, 2):
                if j == 0 and i == 0:
                    continue
                try:
                    item = (self.world['%s;%s' % (x + i, y + j)], x + i, y + j)
                    result.append(item)
                except KeyError:
                    pass
        return result


def solve(n):
    spiral = Spiral()
    result = spiral.find(n)
    print(n, result)


def main():
    solve(15)  # 23
    solve(361527)  # 363010

if __name__ == '__main__':
    main()
