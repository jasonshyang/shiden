use mizuhiki_ta::{core::CandleSeries, indicators::Config as MizuhikiConfig};

use crate::{
    models::{event::InternalEvent, input::InputElement},
    types::{OneShot, StateEngine},
};

#[derive(Debug)]
pub struct BybitStateEngine {
    indicator_config: MizuhikiConfig<f64>,
    candles: CandleSeries<f64>,
}

impl BybitStateEngine {
    pub fn new(timeframe: u64) -> Self {
        BybitStateEngine {
            indicator_config: MizuhikiConfig::default(),
            candles: CandleSeries::new(timeframe),
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

#[async_trait::async_trait]
impl StateEngine<InternalEvent, InputElement> for BybitStateEngine {
    fn name(&self) -> &'static str {
        "bybit_state_engine"
    }

    async fn sync_state(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    #[allow(clippy::single_match)]
    fn process_event(&mut self, event: InternalEvent) -> anyhow::Result<()> {
        match event {
            InternalEvent::BybitTrade(trades) => {
                for trade in trades {
                    self.add_price(trade.price, trade.size, trade.timestamp)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn process_request(&self, request: OneShot<InputElement>) -> anyhow::Result<()> {
        let rsi = self.calc_rsi();
        let natr = self.calc_natr();

        let res = match (rsi, natr) {
            (Some(rsi), Some(natr)) => Some((rsi, natr)),
            _ => None,
        };

        request.respond(InputElement::Bybit(res))?;
        Ok(())
    }

    fn on_shutdown(&mut self) -> anyhow::Result<()> {
        tracing::info!("Shutting down BybitStateEngine");

        println!("Final State:");
        println!("{}", self.candles);
        Ok(())
    }
}
