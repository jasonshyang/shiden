use crate::models::trade::Trade;

#[derive(Debug, Clone)]
pub enum InternalEvent {
    Trade(Trade),
    Error(String),
    Unsupported(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventSource {
    Binance,
    Bybit,
    Coinbase,
}

impl InternalEvent {
    pub fn event_type(&self) -> String {
        match self {
            InternalEvent::Trade(_) => "Trade".to_string(),
            InternalEvent::Error(_) => "Error".to_string(),
            InternalEvent::Unsupported(_) => "Unsupported".to_string(),
        }
    }
}

impl EventSource {
    pub fn get_all() -> Vec<EventSource> {
        vec![
            EventSource::Binance,
            EventSource::Bybit,
            EventSource::Coinbase,
        ]
    }
}
