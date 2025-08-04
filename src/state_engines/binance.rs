use mizuhiki_ta::core::CandleSeries;

use crate::{
    models::event::InternalEvent,
    types::{OneShot, StateEngine},
};

#[derive(Debug)]
pub struct BinanceStateEngine {
    candles: CandleSeries<f64>,
}

impl BinanceStateEngine {
    pub fn new() -> Self {
        BinanceStateEngine {
            candles: CandleSeries::new(1_000),
        }
    }

    pub fn add_price(&mut self, price: f64, volume: f64, timestamp: u64) -> anyhow::Result<()> {
        self.candles.push(price, volume, timestamp)?;
        Ok(())
    }

    pub fn calc_rsi(&self) -> Option<f64> {
        let config = mizuhiki_ta::indicators::Config::default();
        let result = mizuhiki_ta::indicators::rsi_latest(&self.candles, config);

        match result {
            Ok(rsi) => Some(rsi),
            Err(e) => {
                tracing::error!("Error calculating RSI: {}", e);
                None
            }
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
                    self.add_price(trade.price, trade.size, trade.timestamp)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn process_request(&self, request: OneShot<Option<f64>>) -> anyhow::Result<()> {
        let rsi_value = self.calc_rsi();
        request.respond(rsi_value)?;
        Ok(())
    }

    fn on_shutdown(&mut self) -> anyhow::Result<()> {
        tracing::info!("Shutting down BinanceStateEngine");

        println!("Final State:");
        println!("{}", self.candles);
        Ok(())
    }
}
