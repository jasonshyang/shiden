use chrono::DateTime;
use exstreamer::models::{BinanceTrade, BybitTradeData, CoinbaseTicker};

use crate::models::event::EventSource;

#[derive(Debug, Clone)]
pub struct Trade {
    pub source: EventSource,
    pub price: f64,
    pub size: f64,
    pub timestamp: u64,
}

impl TryFrom<BinanceTrade> for Trade {
    type Error = anyhow::Error;

    fn try_from(trade: BinanceTrade) -> Result<Self, Self::Error> {
        match (trade.price.parse::<f64>(), trade.quantity.parse::<f64>()) {
            (Ok(price), Ok(quantity)) => Ok(Trade {
                source: EventSource::Binance,
                price,
                size: quantity,
                timestamp: trade.trade_time,
            }),
            _ => Err(anyhow::anyhow!(
                "Failed to parse Binance trade from string to f64: price='{}', quantity='{}'",
                trade.price,
                trade.quantity
            )),
        }
    }
}

impl TryFrom<BybitTradeData> for Trade {
    type Error = anyhow::Error;

    fn try_from(data: BybitTradeData) -> Result<Self, Self::Error> {
        match (data.price.parse::<f64>(), data.size.parse::<f64>()) {
            (Ok(price), Ok(quantity)) => Ok(Trade {
                source: EventSource::Bybit,
                price,
                size: quantity,
                timestamp: data.timestamp,
            }),
            _ => Err(anyhow::anyhow!(
                "Failed to parse trade from string to f64: price='{}', quantity='{}'",
                data.price,
                data.size
            )),
        }
    }
}

impl TryFrom<CoinbaseTicker> for Trade {
    type Error = anyhow::Error;

    fn try_from(data: CoinbaseTicker) -> Result<Self, Self::Error> {
        match (
            data.price.parse::<f64>(),
            data.last_size.parse::<f64>(),
            parse_iso8601_to_timestamp(&data.time),
        ) {
            (Ok(price), Ok(size), Ok(timestamp)) => Ok(Trade {
                source: EventSource::Coinbase,
                price,
                size,
                timestamp,
            }),
            _ => Err(anyhow::anyhow!(
                "Failed to parse Coinbase trade: price='{}', size='{}', time='{}'",
                data.price,
                data.last_size,
                data.time
            )),
        }
    }
}

fn parse_iso8601_to_timestamp(time_str: &str) -> Result<u64, chrono::ParseError> {
    let datetime = DateTime::parse_from_rfc3339(time_str)?;
    Ok(datetime.timestamp_millis() as u64)
}
