use mizuhiki_ta::{core::CandleSeries, indicators::Config as MizuhikiConfig};

use crate::{
    models::event::InternalEvent,
    types::{OneShot, StateEngine},
};

#[derive(Debug)]
pub struct BinanceStateEngine {
    indicator_config: MizuhikiConfig<f64>,
    candles: CandleSeries<f64>,
}

impl BinanceStateEngine {
    pub fn new() -> Self {
        BinanceStateEngine {
            indicator_config: MizuhikiConfig::default(),
            candles: CandleSeries::new(30_000),
        }
    }

    pub fn add_price(&mut self, price: f64, volume: f64, timestamp: u64) -> anyhow::Result<()> {
        self.candles.push(price, volume, timestamp)?;
        Ok(())
    }

    pub fn calc_rsi(&self) -> Option<f64> {
        let result = mizuhiki_ta::indicators::rsi_latest(&self.candles, &self.indicator_config);

        match result {
            Ok(rsi) => Some(rsi),
            Err(e) => {
                tracing::error!("Error calculating RSI: {}", e);
                None
            }
        }
    }

    pub fn calc_natr(&self) -> Option<f64> {
        let result = mizuhiki_ta::indicators::natr_latest(&self.candles, &self.indicator_config);

        match result {
            Ok(natr) => Some(natr),
            Err(e) => {
                tracing::error!("Error calculating NATR: {}", e);
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
impl StateEngine<InternalEvent, Option<(f64, f64)>> for BinanceStateEngine {
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

    fn process_request(&self, request: OneShot<Option<(f64, f64)>>) -> anyhow::Result<()> {
        let rsi = self.calc_rsi();
        let natr = self.calc_natr();

        let res = match (rsi, natr) {
            (Some(rsi), Some(natr)) => Some((rsi, natr)),
            _ => None,
        };

        request.respond(res)?;
        Ok(())
    }

    fn on_shutdown(&mut self) -> anyhow::Result<()> {
        tracing::info!("Shutting down BinanceStateEngine");

        println!("Final State:");
        println!("{}", self.candles);
        Ok(())
    }
}
