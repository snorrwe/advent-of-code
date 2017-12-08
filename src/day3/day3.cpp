#include "../../core/include/constpair.h"
#include <array>
#include <iostream>
#include <tuple>

class Vector2
{
public:
    constexpr Vector2() : x(0), y(0) {}
    constexpr Vector2(size_t x, size_t y) : x(x), y(y) {}
    constexpr Vector2(Vector2&& v) : x(v.x), y(v.y) {}
    constexpr Vector2(Vector2 const& v) : x(v.x), y(v.y) {}
    constexpr Vector2& operator=(Vector2 const&) = default;

    constexpr Vector2 turnLeft() { return Vector2(-y, x); }
    constexpr size_t distance(Vector2 const& v)
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

constexpr Vector2 operator+(Vector2 const& lhs, Vector2 const& rhs)
{
    return Vector2(lhs.x + rhs.x, lhs.y + rhs.y);
}

constexpr Vector2 operator-(Vector2 const& lhs, Vector2 const& rhs)
{
    return Vector2(lhs.x - rhs.x, lhs.y - rhs.y);
}

constexpr bool operator==(Vector2 const& lhs, Vector2 const& rhs)
{
    return lhs.x == rhs.x && lhs.y == rhs.y;
}

const size_t MAX_SIZE = 100;
using TNode = ConstPair<Vector2, size_t>;
using TSet = std::array<TNode, MAX_SIZE * MAX_SIZE>;

namespace ConstSet
{
typedef ConstPair<TSet, size_t> TAddResult;
typedef ConstPair<bool, TNode> TFindResult;
typedef ConstPair<bool, size_t> TFindIndexResult;

constexpr TFindResult find(TSet const& values, Vector2 const& value, size_t size)
{
    for (int i = 0; i < size; ++i)
    {
        if (value == values[i].first) return TFindResult(true, values[i]);
    }
    return TFindResult(false);
}

constexpr TFindIndexResult findIndex(TSet const& values, Vector2 const& value, size_t size)
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
                auto fresult = ConstSet::find(set, Vector2(v.x + i, v.y + j), setSize);
                if (fresult.first)
                {
                    result[size++] = fresult.second.second;
                }
            }
        }
    }
    return ConstPair<size_t, std::array<size_t, 8>>(size, result);
};

constexpr auto getValueByPosition = [](auto const& spiral, auto size, auto const& position) {
    size_t sum = 0;
    auto nPair = ::neighbours(spiral, position, size);
    auto& neighbours = nPair.second;
    for (int i = 0; i < nPair.first; ++i)
    {
        sum += neighbours[i];
    }
    return sum;
};

constexpr auto solve = [](const int input) {
    auto spiral = TSet({TNode(Vector2(0, 0), 1)});
    size_t size = 1;
    size_t radius = 1;
    Vector2 velocity{0, 1};
    Vector2 position{1, 0};
    size_t current = 0;
    while (current < input)
    {
        // Calculate the value of the current node
        current = getValueByPosition(spiral, size, position);
        // Store the value of the current node
        spiral[size++] = TNode(position, current);
        // Calculate the next position that needs evaluating
        auto next = position + velocity;
        // If next is outside the current edge, it means that we completed that edge
        if (next.distance(Vector2(0, 0)) > radius * 2)
        {
            velocity = velocity.turnLeft(); // Calculate the next velocity
            if (velocity == Vector2(0, 1))
            {
                // We completed a circle, increase the radius
                ++radius;
            }
            else
            {
                // Recalculate the next position
                next = position + velocity;
            }
        }
        position = next;
    }
    return std::make_tuple(current, spiral, size, position);
};

int main(int argc, char const* argv[])
{
    constexpr size_t INPUT = 361527;
    constexpr auto result2 = solve(INPUT); // Expected: 363010
    std::cout << std::get<0>(result2) << std::endl;
    return 0;
}
