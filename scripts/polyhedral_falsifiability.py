import numpy as np
import matplotlib.pyplot as plt

# 1. Define the baseline geometry scaling factor
L0 = 1.441 # femtometers ( fm )

# 2. Define the 12 vertices of the Truncated Tetrahedron
vertices = np.array([
[3 , 1 , 1] , [1 , 3 , 1] , [1 , 1 , 3] , # Group 1 ( Top - Right - Front )
[3 , -1 , -1] , [1 , -3 , -1] , [1 , -1 , -3] , # Group 2 ( Bottom - Right - Back )
[ -1 , 3 , -1] , [ -3 , 1 , -1] , [ -1 , 1 , -3] , # Group 3 ( Top - Left - Back )
[ -1 , -1 , 3] , [ -1 , -3 , 1] , [ -3 , -1 , 1] # Group 4 ( Bottom - Left - Front )
]) * L0

# 3. Calculate the distance matrix between all pairs of vertices
N = 12
distances = np.zeros((N, N))
for i in range(N):
    for j in range(N):
        distances[i, j] = np.linalg.norm(vertices[i] - vertices[j])

# 4. Define the q range ( momentum transfer in fm ^ -1)
q_vals = np.linspace(0.1, 4.0, 400)
form_factor_squared = np.zeros_like(q_vals)

# 5. Calculate the discrete polyhedral form factor | F_tco ( q ) |^2
for k, q in enumerate(q_vals):
    ff_sum = 0
    for i in range(N):
        for j in range(N):
            dist = distances[i, j]
            if dist == 0:
                ff_sum += 1 # sin (0) /0 limit is 1
            else:
                ff_sum += np.sin(q * dist) / (q * dist)
    form_factor_squared[k] = (1 / (N **2)) * ff_sum

# 6. Plot the theoretical curve
plt.figure ( figsize =(12 , 8) )
plt.plot ( q_vals , form_factor_squared , label = ' SpaceTCO Truncated Tetrahedron $ | F_ { tco }( q ) |^2 $ ' , color = 'blue' , linewidth =2)

# Use a log scale for the y - axis , standard for scattering cross - sections
plt.yscale('log')
plt.xlim(0, 4)
plt.ylim(1e-4, 1.0)
plt.axvspan(2.0, 4.0 , color = 'yellow' , alpha =0.1 , label = " High - q Falsification Zone ( Saclay Data Target ) " )
plt.title(" Lead -208 Theoretical Form Factor ( SpaceTCO Geometric Mesh ) ",
fontsize=16)
plt.xlabel(" Momentum Transfer $q$ ( fm$ ^{ -1} $ ) ", fontsize=14)
plt.ylabel(" Scattering Intensity $ | F ( q ) |^2 $ ", fontsize=14)
plt.grid(True, which="both", ls="--", alpha=0.5)
plt.legend(fontsize=12)
plt.tight_layout()
plt.show()