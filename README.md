# Shiden

**Shiden** (紫電) is an algorithmic trading bot framework built in Rust. It's the second iteration on top of [Hayate](https://github.com/jasonshyang/hayate) with a goal to optimize the framework design to achieve parallel state building without shared pointer (lock free), and focus on fair price building across multiple price data source (exchanges).