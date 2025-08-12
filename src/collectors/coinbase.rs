use exstreamer::{error::ExStreamError, models::CoinbaseMessage};
use futures::StreamExt;

use crate::models::{Collector, CollectorStream, InternalEvent};

pub struct CoinbaseCollector;

#[async_trait::async_trait]
impl Collector<InternalEvent> for CoinbaseCollector {
    fn name(&self) -> &'static str {
        "coinbase_collector"
    }

    async fn get_event_stream(&self) -> anyhow::Result<CollectorStream<'_, InternalEvent>> {
        let (stream, _) = exstreamer::StreamBuilder::coinbase()
            .with_trade("BTC-USD")
            .connect()
            .await
            .expect("Failed to create Coinbase streamer");

        let internal_stream = stream.map(coinbase_result_to_internal_event);

        Ok(Box::pin(internal_stream))
    }
}

fn coinbase_result_to_internal_event(
    result: Result<CoinbaseMessage, ExStreamError>,
) -> InternalEvent {
    match result {
        Ok(CoinbaseMessage::Ticker(tick)) => match (*tick).try_into() {
            Ok(trade) => InternalEvent::Trade(trade),
            Err(e) => InternalEvent::Error(e.to_string()),
        },
        Err(e) => InternalEvent::Error(e.to_string()),
        _ => InternalEvent::Unsupported("Not supported".to_string()),
    }
}
