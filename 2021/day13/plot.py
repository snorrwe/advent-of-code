import matplotlib.pyplot as plt 


X = []
Y = []
with open("out.dat", "rb") as f:
    for line in f:
        line = line.decode("utf-8")
        [x,y] = line.split(',')
        X.append(int(x))
        Y.append(int(y))

plt.gca().invert_yaxis()
plt.scatter(x=X, y=Y)
plt.savefig("asd.png")
