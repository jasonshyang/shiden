use std::pin::Pin;

use exstreamer::{error::ExStreamError, models::BybitMessage};
use futures::{Stream, StreamExt, stream};

use crate::models::{Collector, CollectorStream, InternalEvent};

pub struct BybitCollector;

#[async_trait::async_trait]
impl Collector<InternalEvent> for BybitCollector {
    fn name(&self) -> &'static str {
        "bybit_collector"
    }

    async fn get_event_stream(&self) -> anyhow::Result<CollectorStream<'_, InternalEvent>> {
        let (stream, _) = exstreamer::StreamBuilder::bybit()
            .with_trade("btcusdt")
            .connect()
            .await
            .expect("Failed to create Bybit streamer");

        let internal_stream = stream.flat_map(bybit_result_to_internal_event);

        Ok(Box::pin(internal_stream))
    }
}

fn bybit_result_to_internal_event(
    result: Result<BybitMessage, ExStreamError>,
) -> Pin<Box<dyn Stream<Item = InternalEvent> + Send>> {
    match result {
        Ok(BybitMessage::Trade(trades)) => Box::pin(stream::iter(trades.data.into_iter().map(
            |trade| match trade.try_into() {
                Ok(trade) => InternalEvent::Trade(trade),
                Err(e) => InternalEvent::Error(e.to_string()),
            },
        ))),
        Err(e) => Box::pin(stream::once(
            async move { InternalEvent::Error(e.to_string()) },
        )),
        _ => Box::pin(stream::once(async {
            InternalEvent::Unsupported("Not supported".to_string())
        })),
    }
}
