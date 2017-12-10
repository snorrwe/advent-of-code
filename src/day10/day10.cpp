#include "core/word.h"
#include "testing/test.h"
#include <array>
#include <iostream>
#include <tuple>

template <typename ItemsContainer, size_t InputSize> class Day10
{
public:
    constexpr Day10(const char* input) : input(input) {}

    constexpr std::tuple<size_t> solve() const { return std::make_tuple(part1()); }

    constexpr size_t part1() const
    {
        auto numbers = Word::split<InputSize>(input, ',');
        auto items = ItemsContainer();
        for (int i = 0; i < items.size(); ++i)
        {
            items[i] = i;
        }
        size_t skip = 0;
        size_t pos = 0;
        knot(items, numbers, skip, pos);
        return items[0] * items[1];
    }

private:
    constexpr void knot(ItemsContainer& items, Word::SplitResult<InputSize> const& input,
                        size_t& skip, size_t& pos) const
    {
        auto const& size = items.size();
        for (int i = 0; i < input.second; ++i)
        {
            const auto& length = input.first[i].toInt();
            auto affected = ItemsContainer();
            for (int j = pos; j < pos + length; ++j)
            {
                affected[j - pos] = items[j % size];
            }
            auto reverseIt = length - 1;
            for (int j = pos; j < pos + length; ++j)
            {
                items[j % size] = affected[reverseIt--];
            }
            pos = index(pos, length, skip, size);
            skip++;
        }
    }

    constexpr size_t index(size_t pos, size_t length, size_t skip, size_t size) const
    {
        return (pos + length + skip) % size;
    }

    const char* input;
};

void runTests()
{

    Testing::test("Part1, simple constexpr", []() {
        using Container = std::array<size_t, 5>;
        constexpr auto input = ",3,4,1,5";
        constexpr auto day = Day10<Container, 5>(input);
        constexpr auto result = day.solve();
        return Testing::assertEQ(std::get<0>(result), 3 * 4);
    });

    Testing::test("Part1, ", []() {
        using Container = std::array<size_t, 256>;
        constexpr auto input = ",230,1,2,221,97,252,168,169,57,99,0,254,181,255,235,167";
        constexpr auto day = Day10<Container, 256>(input);
        constexpr auto result = day.solve();
        return Testing::assertEQ(std::get<0>(result), 2928);
    });
}

int main(int argc, char const* argv[])
{
    std::cout << "Day10\n";
    runTests();
    return 0;
}
