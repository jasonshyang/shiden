use crate::models::Executor;

#[derive(Debug)]
pub struct EchoExecutor;

#[async_trait::async_trait]
impl Executor<String> for EchoExecutor {
    fn name(&self) -> &'static str {
        "echo_executor"
    }

    async fn execute(&self, action: String) -> anyhow::Result<()> {
        println!("Executing action: {}", action);
        Ok(())
    }
}
