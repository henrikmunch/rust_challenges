import matplotlib.pyplot as plt


def read_file(path):
    with open(path, 'r') as file:
        distances = [float(line.strip()) for line in file]
    return distances


distances = read_file("fittest_python.txt")

plt.plot(distances, marker=',', linewidth=5)
plt.title("History of smallest distances", fontsize=30)
plt.xlabel("Generation", fontsize=30)
plt.ylabel("Distance", fontsize=30)
plt.xticks(fontsize=20)
plt.yticks(fontsize=20)
plt.grid(True)
plt.show()
plt.savefig('history.png')
