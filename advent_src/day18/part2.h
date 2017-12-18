#pragma once
#include "part1.h"
#include <exception>
#include <fstream>
#include <iostream>
#include <map>
#include <queue>
#include <sstream>
#include <stdexcept>
#include <tuple>
#include <vector>

namespace Day18
{

class Tube
{
public:
    typedef long long t_value;

    Tube() : tubes() {}

    void send(int toId, t_value value)
    {
        if (tubes.find(toId) == tubes.end())
        {
            tubes.emplace(toId, std::queue<t_value>());
        }
        tubes[toId].push(value);
    }

    std::tuple<bool, t_value> receive(int toId)
    {
        auto result = !tubes[toId].empty();
        auto value = result ? tubes[toId].front() : 0;
        if (result)
        {
            tubes[toId].pop();
        }
        return std::make_tuple(result, value);
    }

    bool empty()
    {
        for (auto i = tubes.begin(); i != tubes.end(); ++i)
        {
            if (!i->second.empty()) return false;
        }
        return true;
    }
    bool empty(int id) { return tubes[id].empty(); }

private:
    std::map<int, std::queue<t_value>> tubes;
};

class Program
{
public:
    Program(int id, Tube& tube) : id(id), registers(), tube(tube) { registers["p"] = id; }

    size_t run(std::vector<std::string> const& input)
    {
        size_t numberOfSends = 0;
        locked = false;
        while (!locked && pos < input.size())
        {
            auto line = input[pos];
            auto tokens = Token(registers, line);
            switch (tokens.action)
            {
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
            case Action::snd:
                tube.send(1 - id, *tokens.register_value);
                ++numberOfSends;
                break;
            case Action::rcv:
            {
                auto result = tube.receive(id);
                locked = !std::get<0>(result);
                if (!locked)
                {
                    *tokens.register_value = std::get<1>(result);
                }
                else
                {
                    --pos;
                }
                break;
            }
            case Action::jgz:
                if (*tokens.register_value > 0)
                {
                    pos += tokens.value - 1;
                }
                break;
            default:
                throw std::runtime_error("Unrecognised action!");
            }
            ++pos;
        }
        done = pos >= input.size();
        locked = locked || done;
        return numberOfSends;
    }

    const int id;
    bool locked = false;
    bool done = false;

    friend std::ostream& operator<<(std::ostream& stream, Program const& program);

private:
    size_t pos = 0;
    std::map<std::string, long long> registers;
    Tube& tube;
};

std::ostream& operator<<(std::ostream& stream, Program const& program)
{
    stream << "Program # " << program.id << "\n";
    for (auto i = program.registers.begin(); i != program.registers.end(); ++i)
    {
        stream << i->first << " " << i->second << std::endl;
    }
    return stream;
}

constexpr auto part2 = [](const auto& input) {
    Tube tubes{};
    Program p0(0, tubes);
    Program p1(1, tubes);
    uint8_t running = 0;
    size_t result = 0;
    while (!p0.locked || !p1.locked)
    {
        if (running == 0 && !p0.done)
        {
            p0.run(input);
            if (!tubes.empty(1) && !p1.done) p1.locked = false;
        }
        else if (running == 1 && !p1.done)
        {
            auto r = p1.run(input);
            result += r;
            if (!tubes.empty(0) && !p0.done) p0.locked = false;
        }
        running = 1 - running;
    }
    return result;
};
}