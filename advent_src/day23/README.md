# Part2

Pseudo code
```
b = 79;
b *= 100;
b += 100000;
c = b;
c += 17000;
f = 1;
d = 2;
do {
    do {
        e = 2;
        do {
            g = d;
            g *= e;
            g -= b;
            if (!g) 
            {
                f = 0;
            }
            e += 1;
            g = e;
            g -= b;
        } while(g);
        d += 1;
        g = d;
        g -= b;
    } while (g);
    if (!f)
    {
        h += 1;
    }
    g = b;
    g -= c;
    b += 17;
} while(!g);
```

h < 1455549530016