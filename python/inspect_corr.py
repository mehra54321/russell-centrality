import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

# 1) Load your returns (replace the path if yours is different)
rets = pd.read_csv("../data/returns.csv", index_col=0, parse_dates=True)

# 2) Compute the full correlation matrix
corr = rets.corr()

# 3) Pull out the upper‐triangle values (no self-correlations)
mask = np.triu(np.ones(corr.shape, dtype=bool), k=1)
corr_vals = corr.where(mask).stack().values

# 4) Compute key percentiles
for p in [50, 75, 90, 95, 99]:
    print(f"{p}th percentile: {np.percentile(corr_vals, p):.3f}")

# 5) Plot the histogram to see the full shape
plt.hist(corr_vals, bins=50)
plt.title("Distribution of Pairwise Pearson Correlations")
plt.xlabel("Correlation")
plt.ylabel("Frequency")
plt.tight_layout()
plt.show()
