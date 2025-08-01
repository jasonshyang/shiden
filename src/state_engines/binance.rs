use mizuhiki_ta::{core::series::Series, indicators::rsi};

use crate::{
    models::event::InternalEvent,
    types::{OneShot, StateEngine},
};

#[derive(Debug)]
pub struct BinanceStateEngine {
    prices: Series<f64, u64>,
}

impl BinanceStateEngine {
    pub fn new() -> Self {
        BinanceStateEngine {
            prices: Series::new("binance_prices".to_string()),
        }
    }

    pub fn add_price(&mut self, price: f64, timestamp: u64) {
        self.prices.push(price, timestamp);
    }

    pub fn calc_rsi(&self, period: usize) -> Option<f64> {
        let rsi_config = rsi::RsiConfig::<f64>::from_period(period);
        let rsi_result = rsi::rsi(&self.prices, rsi_config);
        if !rsi_result.rsi.is_empty() {
            rsi_result.rsi.values().last().cloned()
        } else {
            None
        }
    }
}

impl Default for BinanceStateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl StateEngine<InternalEvent, Option<f64>> for BinanceStateEngine {
    fn name(&self) -> &'static str {
        "binance_state_engine"
    }

    async fn sync_state(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    #[allow(clippy::single_match)]
    fn process_event(&mut self, event: InternalEvent) -> anyhow::Result<()> {
        match event {
            InternalEvent::Trade(trades) => {
                for trade in trades {
                    self.add_price(trade.price, trade.timestamp);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn process_request(&self, request: OneShot<Option<f64>>) -> anyhow::Result<()> {
        // Process requests if needed, e.g., return the latest RSI value
        let rsi_value = self.calc_rsi(14);
        request.respond(rsi_value)?;
        Ok(())
    }
}
