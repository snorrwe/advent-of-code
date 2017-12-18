#pragma once
#include <exception>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <vector>

namespace Day18
{

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
    typedef long long t_value;
    typedef std::map<std::string, t_value> map;
    Token(map& registers, std::string line)
    {
        using iss_it = std::istream_iterator<std::string>;
        std::istringstream iss(line);
        std::vector<std::string> tokens{iss_it{iss}, iss_it{}};
        action = to_action(tokens[0]);
        registers.try_emplace(tokens[1], 0);
        try
        {
            auto v = std::stoll(tokens[1]);
            registers.try_emplace("tmp", 0);
            registers["tmp"] = v;
            register_value = &(registers.find("tmp")->second);
        }
        catch (std::invalid_argument const& e)
        {
            register_value = &(registers.find(tokens[1])->second);
        }
        if (tokens.size() == 3)
        {
            try
            {
                value = std::stoll(tokens[2]);
            }
            catch (std::invalid_argument const& e)
            {
                registers.try_emplace(tokens[2], 0);
                value = registers[tokens[2]];
            }
        }
    }

    Action action;
    t_value* register_value;
    t_value value;
};

constexpr auto part1 = [](const auto& input) {

    size_t pos = 0;
    long freq = 0;
    std::map<std::string, long long> registers{};
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
}