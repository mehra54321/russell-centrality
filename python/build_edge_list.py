import pandas as pd
import os

def build_edge_list(returns_csv: str, out_path: str, threshold: float = 0.318):
    # 1. Load returns
    r = pd.read_csv(returns_csv, index_col=0, parse_dates=True)

    # 2. Compute correlation matrix
    corr = r.corr()

    # 3. Collect edges above threshold (use absolute value for weight)
    edges = []
    tickers = corr.columns.tolist()
    for i, a in enumerate(tickers):
        for b in tickers[i+1:]:
            w = corr.at[a, b]
            if abs(w) > threshold:
                edges.append((a, b, abs(w)))   # <-- use abs(w) here

    # 4. Save as CSV
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    edge_df = pd.DataFrame(edges, columns=["Source", "Target", "Weight"])
    edge_df.to_csv(out_path, index=False)
    print(f"→ Saved edge list to {out_path} (threshold={threshold}, edges={len(edges)})")

if __name__ == "__main__":
    build_edge_list(
        returns_csv="../data/returns.csv",
        out_path="../data/russ_edges.csv",
        threshold=0.318
    )

