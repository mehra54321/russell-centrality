import pandas as pd
import os

def compute_returns(prices_csv: str, out_path: str):
    # 1. Load prices
    df = pd.read_csv(prices_csv, index_col=0, parse_dates=True)

    # 2. Compute pct-change returns and drop the first NaN row
    returns = df.pct_change().dropna()

    # 3. Ensure the data folder exists
    os.makedirs(os.path.dirname(out_path), exist_ok=True)

    # 4. Save returns
    returns.to_csv(out_path)
    print(f"→ Saved returns to {out_path}")

if __name__ == "__main__":
    compute_returns(
        prices_csv="../data/prices.csv",
        out_path="../data/returns.csv"
    )
