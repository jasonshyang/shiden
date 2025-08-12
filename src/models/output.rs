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

impl PriceData {
    pub fn new(source: EventSource, price: f64, rsi: f64, natr: f64) -> Self {
        Self {
            source,
            price,
            rsi,
            natr,
        }
    }

    pub fn try_new(
        source: EventSource,
        price: Option<f64>,
        rsi: Option<f64>,
        natr: Option<f64>,
    ) -> Option<Self> {
        match (price, rsi, natr) {
            (Some(price), Some(rsi), Some(natr)) => Some(Self {
                source,
                price,
                rsi,
                natr,
            }),
            _ => None,
        }
    }
}
