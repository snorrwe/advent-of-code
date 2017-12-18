# Part2

## Pseudo code

```
// p == 1 || p == 0
if (!p)
{
    a = 2^31 - 1;
    p = 618;
    for i in range(127)
    {
        p = ((p * 8505 % a) * 129749 + 12345) % a;
        b = p;
        b %= 10000;
        send(b);
    }
}
if(!a)
{
    b = receive();
    while(b > 0)
    {
        b = receive();
    }
}
do
{
    f = 0
    i = 126
    while(i)
    {
        a = receive();
        b = receive();
        p = a;
        p *= -1;
        p += b;
        if(!p)
        {
           send(a);
           a = b; 
        }
        else
        {
            send(b);
            f = 1;
            i -= 1;
        }
    }
    send(a);
} while(f || a);
```

I'm not 100% sure about this code, however it helped me find the error in my code which had to do with the "if - else" part.
```
jgz p 4 # if p == 0 skip this next 4 lines
snd a
set a b
jgz 1 3 # else skip the next 3
snd b
set f 1
add i -1
```
