#include <algorithm>
#include <array>
#include <cmath>
#include <iostream>
#include <map>

template <size_t N> using TInput = std::array<unsigned, N>;

auto TEST = TInput<4>{0, 2, 7, 0};
auto ACTUAL = TInput<16>{4, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5};

constexpr auto max_element = [](auto const& values) {
    auto maxi = 0;
    for (int i = 1; i < values.size(); ++i)
    {
        if (values[i] > values[maxi])
        {
            maxi = i;
        }
    }
    return maxi;
};

constexpr auto hash = [](auto const& input) {
    size_t result = 0;
    size_t count = 0;
    for (auto i = input.begin(); i != input.end(); ++i)
    {
        result += *i * pow(10, count++);
    }
    return result;
};

constexpr auto solve = [](auto items, auto callback) {
    std::map<size_t, bool> seen{};
    size_t result = 0;
    while (1)
    {
        ++result;

        auto i = max_element(items);
        auto value = items[i];
        items[i] = 0;
        while (value)
        {
            ++i;
            --value;
            ++items[i % items.size()];
        }

        auto hash = ::hash(items);
        if (seen.find(hash) == seen.end())
        {
            seen[hash] = true;
        }
        else if (callback(hash, result))
        {
            return result;
        }
    }
};

constexpr auto part1 = [](auto hash, auto& result) { return true; };

class part2
{
public:
    bool operator()(size_t hash, size_t& result) const
    {
        if (!seen)
        {
            seen = true;
            lookingfor = hash;
            result = 0;
            return false;
        }
        return lookingfor == hash;
    }

private:
    mutable bool seen = false;
    mutable size_t lookingfor;
};

int main(int argc, char const* argv[])
{
    std::cout << "part1 TEST: " << solve(TEST, part1) << std::endl;
    std::cout << "part1 ACTUAL: " << solve(ACTUAL, part1) << std::endl;
    std::cout << "part2 TEST: " << solve(TEST, part2()) << std::endl;
    std::cout << "part2 ACTUAL: " << solve(ACTUAL, part2()) << std::endl;
    return 0;
}
