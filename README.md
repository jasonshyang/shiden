# Shiden

**Shiden** (紫電) is an algorithmic trading bot framework built in Rust. 

This is my second iteration for desiging a bot framework (see [first iteration](https://github.com/jasonshyang/hayate)), with a goal to optimize the framework design to achieve parallel state building without shared pointer (lock free), and focus on fair price building across multiple price data source (exchanges).

This also serves as a showcase for using two of my other repos (both working in progress): [Exstreamer](https://github.com/jasonshyang/exstreamer) for collecting exchange events, and [Mizuhiki-ta](https://github.com/jasonshyang/mizuhiki-ta) for technical analysis.

## Run

Currently still in active development, the below demo shows an end-to-end price data collection -> state building -> accessing, the end product is RSI and NATR for Binance and Bybit collected in real-time (allow 15 seconds for the 1s candle series to build up)

```bash
cargo run --example demo

# Results
Executing action: Echo Bybit: RSI=15.22346356224048, NATR=0.0021994906284080735
Executing action: Echo Binance: RSI=43.040665461357804, NATR=0.000007078878614003411
Executing action: Echo Bybit: RSI=15.22346356224048, NATR=0.0019062252112869973
Executing action: Echo Binance: RSI=53.57142857142857, NATR=0.0000072970525403528885
Executing action: Echo Bybit: RSI=15.113903270967008, NATR=0.0016636820736086117
Executing action: Echo Binance: RSI=53.57142857142857, NATR=0.000007486137144534947
Executing action: Echo Bybit: RSI=15.113903270967008, NATR=0.0014534765813327512
Executing action: Echo Binance: RSI=41.721864678069096, NATR=0.00000765001113487218
Executing action: Echo Bybit: RSI=15.113903270967008, NATR=0.0012596797038217176
```