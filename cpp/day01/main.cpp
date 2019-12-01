#include <cstdio>
#include <fstream>

int fuel(int mod) { return mod / 3 - 2; }

int part1(std::ifstream& input)
{
    int mod;
    int f = 0;
    while (input >> mod) {
        f += fuel(mod);
    }
    return f;
}

int part2(std::ifstream& input)
{
    int mod;
    int f = 0;
    while (input >> mod) {
        mod = fuel(mod);
        do {
            f += mod;
            mod = fuel(mod);
        } while (mod > 0);
    }
    return f;
}

int main()
{
    std::ifstream f("input.txt");
    printf("Part1: %d\n", part1(f));
    f.close();
    f.open("input.txt");
    printf("Part2: %d\n", part2(f));
}
