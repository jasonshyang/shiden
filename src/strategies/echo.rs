use crate::types::{InputBuilder, Strategy};

#[derive(Debug)]
pub struct EchoStrategy;

#[derive(Debug, Default)]
pub struct EchoInputBuilder {
    data: Option<(f64, f64)>,
}

impl InputBuilder<Option<(f64, f64)>, (f64, f64)> for EchoInputBuilder {
    fn insert(&mut self, data: Option<(f64, f64)>) {
        self.data = data;
    }

    fn build(self) -> Result<(f64, f64), anyhow::Error> {
        self.data.ok_or_else(|| anyhow::anyhow!("No data provided"))
    }
}

impl Strategy<Option<(f64, f64)>, (f64, f64), String> for EchoStrategy {
    type InputBuilder = EchoInputBuilder;
    fn name(&self) -> &'static str {
        "echo_strategy"
    }

    fn interval_ms(&self) -> u64 {
        1000 // 1 second interval
    }

    fn evaluate(&self, input: (f64, f64)) -> Vec<String> {
        vec![
            format!("Echo RSI: {}", input.0),
            format!("Echo NATR: {}", input.1),
        ]
    }
}
