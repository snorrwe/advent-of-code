#include "input.h"
#include "testing/test.h"
#include <exception>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <vector>

enum class Action
{
    snd,
    set,
    add,
    mul,
    mod,
    rcv,
    jgz
};

Action to_action(std::string const& token)
{
    if (token == "snd") return Action::snd;
    if (token == "set") return Action::set;
    if (token == "add") return Action::add;
    if (token == "mul") return Action::mul;
    if (token == "mod") return Action::mod;
    if (token == "rcv") return Action::rcv;
    if (token == "jgz") return Action::jgz;
    throw std::runtime_error("Unrecognised action! " + token);
}

struct Token
{
    typedef std::map<std::string, size_t> map;
    Token(map& registers, std::string line)
    {
        using iss_it = std::istream_iterator<std::string>;
        std::istringstream iss(line);
        std::vector<std::string> tokens{iss_it{iss}, iss_it{}};
        action = to_action(tokens[0]);
        registers.try_emplace(tokens[1], 0);
        register_value = &(registers.find(tokens[1])->second);
        if (tokens.size() == 3)
        {
            try
            {
                value = std::stoi(tokens[2]);
            }
            catch (std::invalid_argument const& e)
            {
                registers.try_emplace(tokens[2], 0);
                value = registers[tokens[2]];
            }
        }
    }

    Action action;
    size_t* register_value;
    size_t value;
};

constexpr auto part1 = [](const auto& input) {

    size_t pos = 0;
    size_t freq = 0;
    std::map<std::string, size_t> registers{};
    while (pos < input.size())
    {
        auto line = input[pos];
        auto tokens = Token(registers, line);
        switch (tokens.action)
        {
        case Action::snd:
            freq = *tokens.register_value;
            break;
        case Action::set:
            *tokens.register_value = tokens.value;
            break;
        case Action::add:
            *tokens.register_value += tokens.value;
            break;
        case Action::mul:
            *tokens.register_value *= tokens.value;
            break;
        case Action::mod:
            *tokens.register_value %= tokens.value;
            break;
        case Action::rcv:
            if (*tokens.register_value != 0)
            {
                return freq;
            }
            break;
        case Action::jgz:
            if (*tokens.register_value > 0)
            {
                pos += tokens.value - 1;
            }
            break;
        default:
            throw std::runtime_error("Unrecognised action!");
        }
        pos++;
    }
    return freq;
};

void testPart1()
{
    Testing::test("Part1, simple", []() {
        auto result = part1(Day18::TEST_INPUT);
        return Testing::assertEQ(result, 4);
    });
}

void runTests()
{
    testPart1();
    /*part2*/
}

void solve()
{
    std::ifstream input("input.txt");
    std::vector<std::string> lines{};
    std::string line;
    while (std::getline(input, line))
    {
        lines.push_back(line);
    }
    std::cout << "Part1: " << part1(lines) << std::endl;
}

int main(int argc, char const* argv[])
{
    std::cout << "Day18\n";
    runTests();
    solve();
    return 0;
}