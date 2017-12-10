#pragma once
#include <cassert>
#include <functional>
#include <iostream>

namespace Testing
{
template <typename T> bool assertEQ(T actual, T expected)
{
    auto result = actual == expected;
    if (!result)
    {
        std::cout << "Failure! Expected: " << expected << " Actual: " << actual << std::endl;
    }
    return result;
}

class Fixture
{
public:
    Fixture(const char* name) : name(name)
    {
        std::cout << "Test Fixture: " << name << " Begin" << std::endl;
    }
    ~Fixture() { std::cout << "Test Fixture: " << name << " End" << std::endl; }

private:
    const char* name;
};

const auto test = [](const char* name, std::function<bool()> t) {
    Fixture f(name);
    return t();
};
}
