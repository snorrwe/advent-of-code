#pragma once
#include <cassert>
#include <functional>
#include <iostream>

namespace Testing
{
template <typename T, typename C> bool assertEQ(T actual, C expected)
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
    Fixture(const char* name) : success(true), name(name)
    {
        std::cout << "Test Fixture: [" << name << "] Begin" << std::endl;
    }
    ~Fixture()
    {
        std::cout << "Test Fixture: [" << name << "]";
        if (this->success)
        {
            std::cout << " Finished";
        }
        else
        {
            std::cout << " Errored";
        }
        std::cout << std::endl;
    }

    void fail() { this->success = false; }

private:
    bool success = true;
    const char* name;
};

bool test(const char* name, std::function<bool()> t)
{
    Fixture f(name);
    auto result = t();
    if (!result)
    {
        f.fail();
    }
    return result;
};
}
