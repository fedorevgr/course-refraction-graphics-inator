from matplotlib import pyplot as plt

data = []
with open("SandBox/data.txt") as f:
    p = list(map(float, f.readline().split()))
    for l in f:
        d = list(map(float, l.split()))
        data.append(d)

fig, ax = plt.subplots()

ax.plot([0, p[0]], [0, p[1]])
ax.scatter([v[0] for v in data], [v[1] for v in data])
ax.set_xlim((-1.2, 1.2))
ax.set_ylim((-1.2, 1.2))
plt.show()