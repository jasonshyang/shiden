use crate::models::event::EventSource;

#[derive(Debug)]
pub enum StateOutput {
    Prices(Vec<PriceData>),
}

#[derive(Debug)]
pub struct PriceData {
    pub source: EventSource,
    pub price: f64,
    pub rsi: f64,
    pub natr: f64,
}

impl std::fmt::Display for PriceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Price: {}, RSI: {}, NATR: {}",
            self.price, self.rsi, self.natr
        )
    }
}
