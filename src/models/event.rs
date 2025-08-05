use exstreamer::models::{BinanceMessage, BybitMessage, BybitTradeData};

#[derive(Debug, Clone)]
pub enum InternalEvent {
    BinanceTrade(Vec<Trade>),
    BybitTrade(Vec<Trade>),
    Error(String),
    Unsupported(String),
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub price: f64,
    pub size: f64,
    pub timestamp: u64,
}

impl From<BinanceMessage> for InternalEvent {
    fn from(message: BinanceMessage) -> Self {
        match message {
            BinanceMessage::Trade(trade) => {
                match (trade.price.parse::<f64>(), trade.quantity.parse::<f64>()) {
                    (Ok(price), Ok(size)) => InternalEvent::BinanceTrade(vec![Trade {
                        price,
                        size,
                        timestamp: trade.trade_time,
                    }]),
                    _ => InternalEvent::Error(format!(
                        "Failed to parse trade from string to f64: price='{}', quantity='{}'",
                        trade.price, trade.quantity
                    )),
                }
            }
            _ => InternalEvent::Unsupported("Unsupported Binance message type".to_string()),
        }
    }
}

impl From<BybitMessage> for InternalEvent {
    fn from(message: BybitMessage) -> Self {
        match message {
            BybitMessage::Trade(trades) => {
                let parsed_trades: Result<Vec<Trade>, String> =
                    trades.data.into_iter().map(TryInto::try_into).collect();

                match parsed_trades {
                    Ok(trades) => InternalEvent::BybitTrade(trades),
                    Err(e) => InternalEvent::Error(e),
                }
            }
            _ => InternalEvent::Unsupported("Unsupported Bybit message type".to_string()),
        }
    }
}

impl TryFrom<BybitTradeData> for Trade {
    type Error = String;

    fn try_from(data: BybitTradeData) -> Result<Self, Self::Error> {
        match (data.price.parse::<f64>(), data.size.parse::<f64>()) {
            (Ok(price), Ok(quantity)) => Ok(Trade {
                price,
                size: quantity,
                timestamp: data.timestamp,
            }),
            _ => Err(format!(
                "Failed to parse trade from string to f64: price='{}', quantity='{}'",
                data.price, data.size
            )),
        }
    }
}
