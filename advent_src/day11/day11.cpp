#include "core/word.h"
#include "input.h"
#include "testing/test.h"
#include <array>
#include <iostream>
#include <tuple>

enum Direction
{
    n,
    ne,
    se,
    s,
    sw,
    nw
};

class Hex
{
public:
    friend std::ostream& operator<<(std::ostream& stream, Hex const& hex);

    constexpr Hex(const int x, const int y, const int z) : x(x), y(y), z(z) {}
    constexpr Hex(Hex const& h) : x(h.x), y(h.y), z(h.z) {}
    constexpr Hex& operator=(Hex const& h)
    {
        x = h.x;
        y = h.y;
        z = h.z;
        return *this;
    }

    constexpr bool operator==(Hex const& other) const
    {
        return x == other.x && y == other.y && z == other.z;
    }
    constexpr bool operator!=(Hex const& other) const { return !(*this == other); }

    constexpr Hex operator+(Hex const& other) const
    {
        return Hex(x + other.x, y + other.y, z + other.z);
    }

    constexpr Hex operator-(Hex const& other) const
    {
        return Hex(x - other.x, y - other.y, z - other.z);
    }

    constexpr int length() const { return (abs(x) + abs(y) + abs(z)) / 2; }

    constexpr int distance(Hex const& other) const { return (*this - other).length(); }

    constexpr Hex neighbour(Direction dir) const
    {
        switch (dir)
        {
        case Direction::n:
            return *this + Hex(1, 0, -1);
        case Direction::ne:
            return *this + Hex(1, -1, 0);
        case Direction::se:
            return *this + Hex(0, -1, 1);
        case Direction::s:
            return *this + Hex(-1, 0, 1);
        case Direction::sw:
            return *this + Hex(-1, 1, 0);
        case Direction::nw:
            return *this + Hex(0, 1, -1);
        }
    }

private:
    constexpr static int abs(int value) { return value >= 0 ? value : -value; }

    int x;
    int y;
    int z;
};

std::ostream& operator<<(std::ostream& stream, Hex const& hex)
{
    stream << "Hex [" << hex.x << ";" << hex.y << ";" << hex.z << "]";
    return stream;
}

constexpr auto solve = [](const char* input) {
    auto start = Hex(0, 0, 0);
    auto current = start;
    size_t part2 = 0;
    auto tokens = Word::split<9000>(input, ',');
    for (int i = 0; i < tokens.second; ++i)
    {
        auto const& word = tokens.first[i];

        auto dir = Direction::s;
        if (word == "ne")
        {
            dir = Direction::ne;
        }
        else if (word == "n")
        {
            dir = Direction::n;
        }
        else if (word == "se")
        {
            dir = Direction::se;
        }
        else if (word == "nw")
        {
            dir = Direction::nw;
        }
        else if (word == "sw")
        {
            dir = Direction::sw;
        }
        current = current.neighbour(dir);
        if (start.distance(current) > part2)
        {
            part2 = start.distance(current);
        }
    }
    auto part1 = start.distance(current);
    return std::make_tuple(part1, part2);
};

void runTests()
{
    Testing::test("Part1 test_simple_same_directions", []() {
        constexpr auto input = "ne,ne,ne";
        constexpr auto result = solve(input);
        return Testing::assertEQ(std::get<0>(result), 3);
    });

    Testing::test("Part1 test_simple_backtrack", []() {
        constexpr auto input = "ne,ne,sw,sw";
        constexpr auto result = solve(input);
        return Testing::assertEQ(std::get<0>(result), 0);
    });

    Testing::test("Part1 test_simple_move_around", []() {
        constexpr auto input = "se,sw,se,sw,sw";
        constexpr auto result = solve(input);
        return Testing::assertEQ(std::get<0>(result), 3);
    });

    Testing::test("Solve", [] {
        constexpr auto result = solve(INPUT);
        return Testing::assertEQ(std::get<0>(result), 705) &&
               Testing::assertEQ(std::get<1>(result), 1469);
    });
}

int main(int argc, char const* argv[])
{
    std::cout << "Day11\n";
    runTests();
    return 0;
}