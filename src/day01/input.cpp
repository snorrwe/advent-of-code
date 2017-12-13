#include <fstream>
#include <iostream>

using namespace std;

int main(int argc, char const* argv[])
{
    ifstream file("input.txt");
    char c;
    while (file >> c)
    {
        int a = c - '0';
        cout << a;
    }
    return 0;
}