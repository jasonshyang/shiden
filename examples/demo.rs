use shiden::{run::run_bot, state_engines::binance::BinanceStateEngine};

#[tokio::main]
async fn main() {
    // Initialize the Echo strategy
    let echo_strategy = shiden::strategies::echo::EchoStrategy;
    let binance_collector = shiden::collectors::binance::BinanceCollector;
    let echo_executor = shiden::executors::echo::EchoExecutor;
    let binance_engine = BinanceStateEngine::new();

    let mut set = run_bot(
        echo_strategy,
        vec![Box::new(binance_engine)],
        vec![Box::new(binance_collector)],
        vec![Box::new(echo_executor)],
    );

    // Wait for all tasks to complete
    while let Some(result) = set.join_next().await {
        match result {
            Ok(_) => tracing::info!("Task completed successfully"),
            Err(e) => tracing::error!("Task failed: {}", e),
        }
    }
}
