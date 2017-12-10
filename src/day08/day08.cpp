#include "../../core/include/constdict.h"
#include "../../core/include/constpair.h"
#include "../../core/include/word.h"
#include "input.h"
#include <array>
#include <iostream>
#include <tuple>

struct Instruction
{
    constexpr Instruction() : target(), amount() {}
    constexpr Instruction(Word const& target, int amount) : target(target), amount(amount) {}

    Word target;
    int amount;
};

template <typename Dict>
constexpr auto actionable =
    [](const auto& registers, size_t size, auto key, auto action, int amount) {
        if (action == Word(">", ">" + 1))
        {
            return Dict::getValue(registers, size, key, 0) > amount;
        }
        if (action == Word("<", "<" + 1))
        {
            return Dict::getValue(registers, size, key, 0) < amount;
        }
        if (action == Word(">=", ">=" + 2))
        {
            return Dict::getValue(registers, size, key, 0) >= amount;
        }
        if (action == Word("<=", "<=" + 2))
        {
            return Dict::getValue(registers, size, key, 0) <= amount;
        }
        if (action == Word("==", "==" + 2))
        {
            return Dict::getValue(registers, size, key, 0) == amount;
        }
        if (action == Word("!=", "!=" + 2))
        {
            return Dict::getValue(registers, size, key, 0) != amount;
        }
        return false;
    };

template <typename Dict>
constexpr auto tokenize = [](const auto& line, auto const& registers, size_t size) {
    auto tokens = Word::split<10>(line);
    auto& words = tokens.first;
    Instruction result(words[0], words[2].toInt());
    if (words[1] == Word("dec", "dec" + 3))
    {
        result.amount *= -1;
    }
    auto execute = actionable<Dict>(registers, size, words[4], words[5], words[6].toInt());
    return ConstPair<bool, Instruction>(execute, result);
};

template <typename Dict>
constexpr auto max = [](auto const& registers, size_t size) {
    auto result = registers[0].second;
    for (int i = 1; i < size; ++i)
    {
        if (registers[i].second > result)
        {
            result = registers[i].second;
        }
    }

    return result;
};

template <typename Dict>
constexpr auto solve = [](const auto& input) {
    auto registers = Dict::create();
    size_t size = 0;
    int max = 0;
    for (auto& line : input)
    {
        auto token = tokenize<Dict>(line, registers, size);
        if (token.first)
        {
            auto& instruction = token.second;
            auto find = Dict::find(registers, size, instruction.target);
            auto amount = find.second.second;
            if (!find.first)
            {
                registers = Dict::add(registers, size, instruction.target, 0).first;
                size++;
                amount = 0;
            }
            amount += instruction.amount;
            // Part 2
            if (amount > max)
            {
                max = amount;
            }
            registers = Dict::mutate(registers, size, instruction.target, amount).first;
        }
    }
    return std::make_tuple(::max<Dict>(registers, size), max);
};

int main(int argc, char const* argv[])
{
    constexpr auto testResult = solve<TestDict>(TEST);
    std::cout << "Test result 1: " << std::get<0>(testResult) << std::endl;
    std::cout << "Test result 2: " << std::get<1>(testResult) << std::endl;
    auto result = solve<ActualDict>(ACTUAL);
    std::cout << "Actual result 1: " << std::get<0>(result) << std::endl; // Expected: 7296
    std::cout << "Actual result 2: " << std::get<1>(result) << std::endl; // Expected: 8186
    return 0;
}
