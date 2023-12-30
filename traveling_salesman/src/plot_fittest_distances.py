import matplotlib.pyplot as plt


def read_file(path):
    with open(path, 'r') as file:
        distances = [float(line.strip()) for line in file]
    return distances


distances = read_file("fittest_python.txt")

plt.plot(distances, marker=',')
plt.title("History of fittest distances")
plt.xlabel("Generation")
plt.ylabel("Distance")
plt.grid(True)
plt.show()
