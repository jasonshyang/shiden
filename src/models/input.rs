#[derive(Debug)]
pub enum InputElement {
    Binance(Option<(f64, f64)>), // (RSI, NATR)
    Bybit(Option<(f64, f64)>),   // (RSI, NATR)
}
