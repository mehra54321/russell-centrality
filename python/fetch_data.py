import pandas as pd
import yfinance as yf
import os

# 1) Read your 1 000 tickers from data/NASDAQ_100.csv
tickers = pd.read_csv("/Users/atharvamehra/Documents/nasdaq-centrality/data/russ_1000.csv")["Ticker"].tolist()

def fetch_prices(tickers, start, end, out_path):
    df = yf.download( tickers, start=start, end=end, progress=False )["Close"]
    df = df.dropna(axis=1)
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    df.to_csv(out_path)
    print(f"→ Saved prices to {out_path}")
    print("Columns:", df.columns.tolist())

if __name__ == "__main__":
    fetch_prices(
        tickers,
        start="2023-04-01",
        end="2024-04-01",
        out_path="../data/prices.csv"
    )
