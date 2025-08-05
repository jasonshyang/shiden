use futures::StreamExt;

use crate::{
    models::event::InternalEvent,
    types::{Collector, CollectorStream},
};

pub struct BybitCollector;

#[async_trait::async_trait]
impl Collector<InternalEvent> for BybitCollector {
    fn name(&self) -> &'static str {
        "bybit_collector"
    }

    async fn get_event_stream(&self) -> anyhow::Result<CollectorStream<'_, InternalEvent>> {
        let (stream, _) = exstreamer::StreamBuilder::bybit()
            .trade("btcusdt")
            .connect()
            .await
            .expect("Failed to create Bybit streamer");

        let internal_stream = stream.map(|msg| match msg {
            Ok(bybit_msg) => InternalEvent::from(bybit_msg),
            Err(e) => InternalEvent::Error(e.to_string()),
        });

        Ok(Box::pin(internal_stream))
    }
}
