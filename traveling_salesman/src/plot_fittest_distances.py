import matplotlib.pyplot as plt


def read_file(path):
    with open(path, 'r') as file:
        distances = [float(line.strip()) for line in file]
    return distances


distances = read_file("fittest_python.txt")

plt.plot(distances, marker=',', linewidth=5)
plt.title("History of smallest distances", fontsize=20)
plt.xlabel("Generation", fontsize=20)
plt.ylabel("Distance", fontsize=20)
plt.xticks(fontsize=10)
plt.yticks(fontsize=10)
plt.grid(True)
plt.savefig('history.png')
plt.show()
