#include "input.h"
#include <iostream>
#include <tuple>

enum class State
{
    Default,
    Garbage,
    Ignore
};

constexpr auto solve = [](auto const& input) {
    size_t score = 0 /*part1*/, dumped = 0; /*part2*/
    int depth = 0;
    auto state = State::Default;
    for (auto i = input; *i; ++i)
    {
        switch (state)
        {
        case State::Ignore:
            state = State::Garbage;
            continue;
        case State::Default:
            switch (*i)
            {
            case '{':
                ++depth;
                score += depth;
                break;
            case '}':
                --depth;
                break;
            case '<':
                state = State::Garbage;
                break;
            }
            break;
        case State::Garbage:
            switch (*i)
            {
            case '>':
                state = State::Default;
                break;
            case '!':
                state = State::Ignore;
                break;
            default:
                ++dumped;
                break;
            }
            break;
        }
    }
    return std::make_tuple(score, dumped);
};

int main(int argc, char const* argv[])
{
    constexpr auto result = solve(INPUT);
    std::cout << "Part1: " << std::get<0>(result) << std::endl; // Expected: 12505
    std::cout << "Part2: " << std::get<1>(result) << std::endl; // Expected: 6671
    return 0;
}
