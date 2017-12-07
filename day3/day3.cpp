#include <array>
#include <iostream>

class Vector
{
public:
    constexpr Vector() : x(0), y(0) {}
    constexpr Vector(size_t x, size_t y) : x(x), y(y) {}
    constexpr Vector(Vector&& v) : x(v.x), y(v.y) {}
    constexpr Vector(Vector const& v) : x(v.x), y(v.y) {}
    constexpr Vector& operator=(Vector const&) = default;

    constexpr Vector turnLeft() { return Vector(-y, x); }
    constexpr size_t distance(Vector const& v)
    {
        auto x = this->x - v.x;
        auto y = this->y - v.y;
        if (x < 0) x = -x;
        if (y < 0) y = -y;
        return x + y;
    }

    int x;
    int y;
};

constexpr Vector operator+(Vector const& lhs, Vector const& rhs)
{
    return Vector(lhs.x + rhs.x, lhs.y + rhs.y);
}

constexpr Vector operator-(Vector const& lhs, Vector const& rhs)
{
    return Vector(lhs.x - rhs.x, lhs.y - rhs.y);
}

constexpr bool operator==(Vector const& lhs, Vector const& rhs)
{
    return lhs.x == rhs.x && lhs.y == rhs.y;
}

template <typename T1, typename T2> class ConstPair
{
public:
    constexpr ConstPair() : first(), second() {}
    constexpr ConstPair(ConstPair<T1, T2> const& pair) : first(pair.first), second(pair.second) {}
    constexpr ConstPair(T1 const& first, T2 const& second) : first(first), second(second) {}
    constexpr ConstPair(T1 const& first) : first(first) {}

    constexpr ConstPair<T1, T2>& operator=(ConstPair<T1, T2> const& other)
    {
        first = other.first;
        second = other.second;
        return *this;
    }

    T1 first;
    T2 second;
};

const size_t MAX_SIZE = 100;
using TNode = ConstPair<Vector, size_t>;
using TSet = std::array<TNode, MAX_SIZE * MAX_SIZE>;

namespace ConstSet
{
typedef ConstPair<TSet, size_t> TAddResult;
typedef ConstPair<bool, TNode> TFindResult;
typedef ConstPair<bool, size_t> TFindIndexResult;

constexpr TFindResult find(TSet const& values, Vector const& value, size_t size)
{
    for (int i = 0; i < size; ++i)
    {
        if (value == values[i].first) return TFindResult(true, values[i]);
    }
    return TFindResult(false);
}

constexpr TFindIndexResult findIndex(TSet const& values, Vector const& value, size_t size)
{
    for (int i = 0; i < size; ++i)
    {
        if (value == values[i].first) return TFindIndexResult(true, i);
    }
    return TFindIndexResult(false, -1);
}

constexpr TAddResult add(TSet const& values, TNode const& value, size_t size)
{
    auto fresult = findIndex(values, value.first, size);
    auto result = TSet(values);
    if (!fresult.first)
    {
        result[size++] = value;
    }
    return TAddResult(result, size);
}
};

constexpr auto neighbours = [](auto const& set, auto const& v, auto setSize) {
    std::array<size_t, 8> result{};
    size_t size = 0;
    for (int i = -1; i <= 1; ++i)
    {
        for (int j = -1; j <= 1; ++j)
        {
            if (i != 0 || j != 0)
            {
                auto fresult = ConstSet::find(set, Vector(v.x + i, v.y + j), setSize);
                if (fresult.first)
                {
                    result[size++] = fresult.second.second;
                }
            }
        }
    }
    return ConstPair<size_t, std::array<size_t, 8>>(size, result);
};

constexpr auto solve = [](auto const& input) {
    auto set = TSet({TNode(Vector(0, 0), 1)});
    size_t size = 1;
    size_t current = 0;
    size_t radius = 1;
    Vector velocity{0, 1};
    Vector position{1, 0};
    while (current <= input)
    {
        size_t sum = 0;
        auto nPair = ::neighbours(set, position, size);
        auto& neighbours = nPair.second;
        for (int i = 0; i < nPair.first; ++i)
        {
            sum += neighbours[i];
        }
        current = sum;
        set[size++] = TNode(position, current);
        auto next = position + velocity;
        auto distance = next.distance(Vector(0, 0));
        if (distance < radius || distance > radius * 2)
        {
            velocity = velocity.turnLeft();
            if (velocity == Vector(0, 1))
            {
                position = Vector(position.x + 1, position.y);
                ++radius;
            }
            else
            {
                next = position + velocity;
            }
        }
        position = next;
    }
    return current;
};

int main(int argc, char const* argv[])
{
    // auto result = solve(11); // Expected: 23 // Comiles and runs correctly
    constexpr auto result = solve(11); // Expected: 23 // Crashes during compilation with clang
    // constexpr auto result = solve(361527); // Expected: 363010
    std::cout << result << std::endl;
    return 0;
}
