use futures::StreamExt;

use crate::{
    models::event::InternalEvent,
    types::{Collector, CollectorStream},
};

pub struct BinanceCollector;

#[async_trait::async_trait]
impl Collector<InternalEvent> for BinanceCollector {
    fn name(&self) -> &'static str {
        "binance_collector"
    }

    async fn get_event_stream(&self) -> anyhow::Result<CollectorStream<'_, InternalEvent>> {
        let (stream, _) = exstreamer::StreamBuilder::binance()
            .trade("btcusdt")
            .connect()
            .await
            .expect("Failed to create Binance streamer");

        let internal_stream = stream.map(|msg| match msg {
            Ok(binance_msg) => InternalEvent::from(binance_msg),
            Err(e) => InternalEvent::Error(e.to_string()),
        });

        Ok(Box::pin(internal_stream))
    }
}
