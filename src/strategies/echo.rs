use crate::models::{InputBuilder, PriceData, StateOutput, Strategy};

#[derive(Debug)]
pub struct EchoStrategy;

#[derive(Debug)]
pub struct EchoInput {
    prices: Vec<PriceData>,
}

#[derive(Debug, Default)]
pub struct EchoInputBuilder {
    state_output: Option<StateOutput>,
}

impl InputBuilder<StateOutput, EchoInput> for EchoInputBuilder {
    fn insert(&mut self, data: StateOutput) {
        self.state_output = Some(data);
    }

    fn build(self) -> Result<EchoInput, anyhow::Error> {
        match self.state_output {
            Some(StateOutput::Prices(prices)) => Ok(EchoInput { prices }),
            _ => Err(anyhow::anyhow!("No prices available in state output")),
        }
    }
}

impl Strategy<StateOutput, EchoInput, String> for EchoStrategy {
    type InputBuilder = EchoInputBuilder;
    fn name(&self) -> &'static str {
        "echo_strategy"
    }

    fn interval_ms(&self) -> u64 {
        1000 // 1 second interval
    }

    fn evaluate(&self, input: EchoInput) -> Vec<String> {
        input
            .prices
            .iter()
            .map(|price_data| format!("{:?}: {}", price_data.source, price_data))
            .collect()
    }
}
