#include "core/constpair.h"
#include "core/constset.h"
#include "core/vector2.h"
#include <array>
#include <iostream>
#include <tuple>

const size_t MAX_SIZE = 100;
using TNode = ConstPair<Vector2, size_t>;
using TSet = std::array<TNode, MAX_SIZE * MAX_SIZE>;

constexpr auto neighbours = [](auto const& set, auto const& v, auto setSize) {
    std::array<size_t, 8> result{};
    size_t size = 0;
    for (int i = -1; i <= 1; ++i)
    {
        for (int j = -1; j <= 1; ++j)
        {
            if (i != 0 || j != 0)
            {
                auto fresult = ConstSet::find<TSet, TNode>(set, Vector2(v.x + i, v.y + j), setSize);
                if (fresult.first)
                {
                    result[size++] = fresult.second.second;
                }
            }
        }
    }
    return ConstPair<size_t, std::array<size_t, 8>>(size, result);
};

constexpr auto getValueByPosition = [](auto const& spiral, const size_t size,
                                       auto const& position) {
    size_t sum = 0;
    auto nPair = ::neighbours(spiral, position, size);
    auto& neighbours = nPair.second;
    for (int i = 0; i < nPair.first; ++i)
    {
        sum += neighbours[i];
    }
    return sum;
};

constexpr auto solve = [](const size_t input) {
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
    std::cout << "Result2: " << std::get<0>(result2) << std::endl;
    return 0;
}
