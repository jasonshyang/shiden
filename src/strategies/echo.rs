use crate::{
    models::input::InputElement,
    types::{InputBuilder, Strategy},
};

#[derive(Debug)]
pub struct EchoInput {
    binance: (f64, f64), // (RSI, NATR)
    bybit: (f64, f64),   // (RSI, NATR)
}

#[derive(Debug)]
pub struct EchoStrategy;

#[derive(Debug)]
pub struct EchoInputBuilder {
    binance: InputElement,
    bybit: InputElement,
}

impl Default for EchoInputBuilder {
    fn default() -> Self {
        EchoInputBuilder {
            binance: InputElement::Binance(None),
            bybit: InputElement::Bybit(None),
        }
    }
}

impl InputBuilder<InputElement, EchoInput> for EchoInputBuilder {
    fn insert(&mut self, data: InputElement) {
        match data {
            InputElement::Binance(_) => self.binance = data,
            InputElement::Bybit(_) => self.bybit = data,
        }
    }

    fn build(self) -> Result<EchoInput, anyhow::Error> {
        match (self.binance, self.bybit) {
            (InputElement::Binance(Some(binance)), InputElement::Bybit(Some(bybit))) => {
                Ok(EchoInput { binance, bybit })
            }
            _ => Err(anyhow::anyhow!("Incomplete input data")),
        }
    }
}

impl Strategy<InputElement, EchoInput, String> for EchoStrategy {
    type InputBuilder = EchoInputBuilder;
    fn name(&self) -> &'static str {
        "echo_strategy"
    }

    fn interval_ms(&self) -> u64 {
        1000 // 1 second interval
    }

    fn evaluate(&self, input: EchoInput) -> Vec<String> {
        vec![
            format!(
                "Echo Binance: RSI={}, NATR={}",
                input.binance.0, input.binance.1
            ),
            format!("Echo Bybit: RSI={}, NATR={}", input.bybit.0, input.bybit.1),
        ]
    }
}
