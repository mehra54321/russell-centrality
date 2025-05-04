import pandas as pd

# 1) Load your results
df = pd.read_csv("/Users/atharvamehra/Documents/nasdaq-centrality/data/centrality_scores.csv")

# 2) How many tickers survived?
print("Total tickers:", len(df))

# 3) Top 10 by Degree
print("\nTop 10 by Degree:")
print(df.nlargest(10, "Degree")[["Ticker", "Degree"]].to_string(index=False))

# 4) Top 10 by Closeness
print("\nTop 10 by Closeness:")
print(df.nlargest(10, "Closeness")[["Ticker", "Closeness"]].to_string(index=False))

# 5) Bottom 10 (least connected)
print("\nBottom 10 by Degree:")
print(df.nsmallest(10, "Degree")[["Ticker", "Degree"]].to_string(index=False))


import matplotlib.pyplot as plt

# Degree histogram
plt.hist(df["Degree"], bins=30)
plt.title("Degree Distribution (n≈650)")
plt.xlabel("Degree")
plt.ylabel("Frequency")
plt.tight_layout()
plt.show()

# Closeness histogram
plt.hist(df["Closeness"], bins=30)
plt.title("Closeness Distribution (n≈650)")
plt.xlabel("Closeness")
plt.ylabel("Frequency")
plt.tight_layout()
plt.show()
