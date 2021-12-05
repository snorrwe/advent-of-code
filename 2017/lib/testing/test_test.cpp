#include "test.h"

int main(int argc, char const* argv[])
{
    Testing::test("Test1", []() { return true; });
    return 0;
}