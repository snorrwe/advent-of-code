#include "input.h"
#include "part1.h"
#include "part2.h"
#include "testing/test.h"
#include <fstream>
#include <iostream>
#include <vector>

using namespace Day18;

bool testPart1()
{
    return Testing::test("Part1, simple", []() {
        auto result = part1(TEST_INPUT);
        return Testing::assertEQ(result, 4);
    });
}

bool testPart2()
{
    return Testing::test("Part2, simple", []() {
        std::vector<std::string> input{};
        for (auto& line : TEST_INPUT2)
        {
            input.push_back(line);
        }
        auto result = part2(input);
        return Testing::assertEQ(result, 3);
    });
}

bool runTests() { return testPart1() && testPart2(); }

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
    std::cout << "Part2: " << part2(lines) << std::endl;
}

int main(int argc, char const* argv[])
{
    std::cout << "Day18\n";
    if (runTests()) solve();
    return 0;
}