from math import sqrt
​
h = 0
b = 79
b *= 100
b += 100000
c = b
c += 17000;
for i in range(b, c+1, 17):
  f = True
  for e in range(2, int(sqrt(i))):
      if i % e == 0:
        f = False
        break
  if not f:
    h += 1
  print(i, h)
print(h)
