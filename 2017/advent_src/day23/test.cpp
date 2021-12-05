#include <iostream>

int main(int argc, char const* argv[])
{
    size_t a = 0, b = 0, c = 0, d = 0, e = 0, f = 0, g = 0, h = 0;
    b = 79;
    b *= 100;
    b += 100000;
    c = b;
    c += 17000;
    do
    {
        f = 1;
        d = 2;
        do
        {
            e = 2;
            do
            {
                if (d * e == b)
                {
                    f = 0;
                }
                e += 1;
            } while (e != b);
            d += 1;
        } while (d != b);
        if (!f)
        {
            h += 1;
        }
        b += 17;
    } while (c != b);
    std::cout << h << std::endl;
    return 0;
}