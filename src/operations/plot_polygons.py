import matplotlib.pyplot as plt

# Define the list of vectors
left_shape = [(0.55, 0.45), (0.45, 0.45), (0.45, 0.55), (0.55, 0.55)]
right_shape = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]

# Function to plot the polygons
def plot_polygon(vertices, color, label):
    x, y = zip(*vertices + [vertices[0]])  # Close the shape
    plt.plot(x, y, color=color, marker='o', label=label)

# Plot the polygons
plt.figure(figsize=(6, 6))
plot_polygon(left_shape, 'b', 'Left Shape')
plot_polygon(right_shape, 'r', 'Right Shape')

# Formatting
plt.axhline(0, color='black', linewidth=0.5)
plt.axvline(0, color='black', linewidth=0.5)
plt.grid(True, linestyle='--', linewidth=0.5)
plt.legend()
plt.title("Polygons from Given Vectors")
plt.show()
