#include "core/constpair.h"
#include "core/word.h"
#include "testing/test.h"
#include <array>
#include <iostream>
#include <tuple>

template <typename ItemsContainer, size_t InputSize> class Day10
{
public:
    using Part2Result = std::array<char, 32>;

    constexpr Day10(const char* input) : input(input) {}

    constexpr size_t part1() const
    {
        auto input = Word::split<InputSize>(this->input, ',');
        auto numbers = ItemsContainer();
        for (int i = 0; i < input.second; ++i)
        {
            numbers[i] = input.first[i].toInt();
        }
        auto items = initItems();
        size_t skip = 0;
        size_t pos = 0;
        knot(items, ConstPair(numbers, input.second), skip, pos);
        return items[0] * items[1];
    }

    constexpr Part2Result part2() const
    {
        auto numbers = ItemsContainer();
        size_t size = 0;
        auto i = input;
        while (*i)
        {
            numbers[size++] = *i;
            ++i;
        }
        const auto padding = std::array<size_t, 5>{17, 31, 73, 47, 23};
        for (int i = 0; i < padding.size(); ++i)
        {
            numbers[size++] = padding[i];
        }
        size_t skip = 0;
        size_t pos = 0;
        auto items = initItems();
        for (int i = 0; i < 64; ++i)
        {
            knot(items, ConstPair(numbers, size), skip, pos);
        }
        auto result = Part2Result();
        size = 0;
        for (int i = 0; i < 256; i += 16)
        {
            auto xorResult = items[i];
            for (auto j = i + 1; j < i + 16; ++j)
            {
                xorResult = xorResult ^ items[j];
            }
            if (xorResult < 16)
            {
                result[size++] = toHex(0);
                result[size++] = toHex(xorResult);
            }
            else
            {
                auto first = xorResult / 16;
                result[size++] = toHex(first);
                result[size++] = toHex(xorResult - (first * 16));
            }
        }
        return result;
    }

    constexpr char toHex(char value) const
    {
        if (value < 10) return '0' + value;
        return 'a' + (value - 10);
    }

private:
    constexpr void knot(ItemsContainer& items, ConstPair<ItemsContainer, size_t> const& input,
                        size_t& skip, size_t& pos) const
    {
        auto const& size = items.size();
        for (int i = 0; i < input.second; ++i)
        {
            const auto& length = input.first[i];
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

    constexpr ItemsContainer initItems() const
    {
        auto items = ItemsContainer();
        for (int i = 0; i < items.size(); ++i)
        {
            items[i] = i;
        }
        return items;
    }

    const char* input;
};

void testPart1()
{
    Testing::test("Part1, simple constexpr", []() {
        using Container = std::array<size_t, 5>;
        constexpr auto input = ",3,4,1,5";
        constexpr auto day = Day10<Container, 5>(input);
        constexpr auto result = day.part1();
        return Testing::assertEQ(result, 3 * 4);
    });

    Testing::test("Part1", []() {
        using Container = std::array<size_t, 256>;
        constexpr auto input = ",230,1,2,221,97,252,168,169,57,99,0,254,181,255,235,167";
        constexpr auto day = Day10<Container, 256>(input);
        constexpr auto result = day.part1();
        std::cout << result << std::endl;
        return Testing::assertEQ(result, 2928);
    });
}

void testPart2()
{
    using Container = std::array<size_t, 256>;
    using Day10 = Day10<Container, 256>;

    Testing::test("Part2, simple, empty", []() {
        constexpr auto input = "";
        constexpr auto day = Day10(input);
        constexpr auto result = day.part2();
        return Testing::assertEQ(result, "a2582a3a0e66e6e86e3812dcb672a272");
    });

    Testing::test("Part2, simple, AoC", []() {
        constexpr auto input = "AoC 2017";
        constexpr auto day = Day10(input);
        constexpr auto result = day.part2();
        return Testing::assertEQ(result, "33efeb34ea91902bb2f59c9920caa6cd");
    });

    Testing::test("Part2", []() {
        constexpr auto input = "230,1,2,221,97,252,168,169,57,99,0,254,181,255,235,167";
        constexpr auto day = Day10(input);
        constexpr auto result = day.part2();
        std::cout << result << std::endl;
        return Testing::assertEQ(result, "0c2f794b2eb555f7830766bf8fb65a16");
    });
}

void runTests()
{
    testPart1();
    testPart2();
}

int main(int argc, char const* argv[])
{
    std::cout << "Day10\n";
    runTests();
    return 0;
}
