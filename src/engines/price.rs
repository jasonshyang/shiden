use std::collections::HashMap;

use mizuhiki_ta::{core::CandleSeries, indicators::Config as MizuhikiConfig};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    metrics::{BotMetrics, DurationRecorder},
    models::{
        event::{EventSource, InternalEvent},
        output::{PriceData, StateOutput},
        trade::Trade,
        traits::{OneShot, StateEngine},
    },
};

#[derive(Debug)]
pub struct PriceStateEngine {
    indicator_config: MizuhikiConfig<f64>,
    candles: HashMap<EventSource, CandleSeries<f64>>,
}

#[async_trait::async_trait]
impl StateEngine<InternalEvent, StateOutput> for PriceStateEngine {
    fn name(&self) -> &'static str {
        "price_state_engine"
    }

    async fn sync_state(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn process_event(&mut self, event: InternalEvent) -> anyhow::Result<()> {
        let recorder = DurationRecorder::start();
        let event_type = event.event_type();

        match event {
            InternalEvent::Trade(trade) => {
                self.add_trade(trade)?;
            }
            InternalEvent::Error(e) => {
                tracing::error!("Error processing Binance event: {}", e);
                return Err(anyhow::anyhow!("Error processing Binance event: {}", e));
            }
            _ => {}
        }

        let duration = recorder.end();
        BotMetrics::record_event_processing(self.name(), &event_type, duration);

        let candle_size = self.candles.len();
        BotMetrics::record_candle_series_size(self.name(), candle_size);

        Ok(())
    }

    fn process_request(&self, request: OneShot<StateOutput>) -> anyhow::Result<()> {
        let data = self
            .candles
            .par_iter()
            .filter_map(|(source, candle_series)| {
                let rsi = self.calc_rsi(source);
                let natr = self.calc_natr(source);
                let price = candle_series.closes().last().cloned();
                PriceData::try_new(source.clone(), price, rsi, natr)
            })
            .collect::<Vec<PriceData>>();

        request.respond(StateOutput::Prices(data))?;
        Ok(())
    }

    fn on_shutdown(&mut self) -> anyhow::Result<()> {
        tracing::info!("Shutting down BinanceStateEngine");

        println!("Final State:");
        for (source, candle_series) in &self.candles {
            println!("Source: {:?}, Candles: {}", source, candle_series.len());
        }
        Ok(())
    }
}

impl PriceStateEngine {
    pub fn new(timeframe: u64) -> Self {
        let candles = EventSource::get_all()
            .into_iter()
            .map(|source| (source, CandleSeries::new(timeframe)))
            .collect();
        PriceStateEngine {
            indicator_config: MizuhikiConfig::default(),
            candles,
        }
    }

    pub fn add_trade(&mut self, trade: Trade) -> anyhow::Result<()> {
        match self.candles.get_mut(&trade.source) {
            Some(candle_series) => {
                candle_series.push(trade.price, trade.size, trade.timestamp)?;
            }
            None => {
                return Err(anyhow::anyhow!(
                    "No candle series found for source: {:?}",
                    trade.source
                ));
            }
        }
        Ok(())
    }

    pub fn calc_rsi(&self, source: &EventSource) -> Option<f64> {
        let result =
            mizuhiki_ta::indicators::rsi_latest(&self.candles[source], &self.indicator_config);

        match result {
            Ok(rsi) => Some(rsi),
            Err(e) => {
                tracing::debug!("Error calculating RSI: {}", e);
                None
            }
        }
    }

    pub fn calc_natr(&self, source: &EventSource) -> Option<f64> {
        let result =
            mizuhiki_ta::indicators::natr_latest(&self.candles[source], &self.indicator_config);

        match result {
            Ok(natr) => Some(natr),
            Err(e) => {
                tracing::debug!("Error calculating NATR: {}", e);
                None
            }
        }
    }
}
