import yfinance as yf
import pandas as pd

df = yf.download("AAPL", start="2023-04-01", end="2024-04-01")['Adj Close']
print(df.head())
print(df.shape)
df.to_csv("data/test_aapl.csv")
