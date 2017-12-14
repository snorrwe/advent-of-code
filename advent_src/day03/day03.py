#!usr/bin/python


class Cross(object):

    def __init__(self, begin, end, length):
        self.begin = begin
        self.end = end
        self.length = length

        l = self.length - 1
        self.right = self.begin + l // 2
        self.top = self.right + l
        self.left = self.top + l
        self.bottom = self.left + l

    def __getitem__(self, index):
        return {
            0: self.right,
            1: self.top,
            2: self.left,
            3: self.bottom
        }[index % 4]


class Edge(object):

    def __init__(self, n):
        self.n = n

    def __find_begin_end(self):
        """
        [begin, end)
        begin <= n <= end
        """
        self.length = 1
        self.end = 1
        self.begin = 1
        while self.end <= self.n:
            self.length += 2
            self.begin = self.end
            self.end += self.length * 4 - 4
        return (self.begin, self.end, self.length)

    def __find_edge(self):
        """
           1
        -----
        |   | 0
      2 |   |
        ----
          3

        """
        self.edge = 0
        for i in range(self.begin + self.length - 1,
                       self.end,
                       self.length - 1):
            if i >= self.n:
                break
            self.edge += 1
        return self.edge

    def __find_quarter(self):
        """
          1  |  0
        ----------
          2 |  3

        """
        self.cross = Cross(self.begin, self.end, self.length)
        if self.cross.right <= self.n < self.cross.top:
            self.quarter = 0
        elif self.cross.top <= self.n < self.cross.left:
            self.quarter = 1
        elif self.cross.left <= self.n < self.cross.bottom:
            self.quarter = 2
        else:
            self.quarter = 3

    def find(self):
        self.__find_begin_end()
        self.__find_edge()
        self.__find_quarter()
        return (self.begin,
                self.end,
                self.length,
                self.edge,
                self.quarter,
                self.cross)


def find_edge(n):
    return Edge(n).find()


def solve(n):
    begin, end, length, edge, quarter, cross = find_edge(n)
    closest_cross_point = cross[edge]

    radius = (length - 1) // 2
    result = abs(closest_cross_point - n) + radius

    print(n, result)


def main():
    solve(361527)

if __name__ == '__main__':
    main()
