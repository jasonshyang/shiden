use shiden::{
    run::run_bot,
    state_engines::{binance::BinanceStateEngine, bybit::BybitStateEngine},
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize the Echo strategy
    let echo_strategy = shiden::strategies::echo::EchoStrategy;
    let binance_collector = shiden::collectors::binance::BinanceCollector;
    let bybit_collector = shiden::collectors::bybit::BybitCollector;
    let echo_executor = shiden::executors::echo::EchoExecutor;
    let binance_engine = BinanceStateEngine::new(1_000); // 1 second timeframe
    let bybit_engine = BybitStateEngine::new(1_000); // 1 second timeframe
    let shutdown = CancellationToken::new();

    let mut set = run_bot(
        echo_strategy,
        vec![Box::new(binance_engine), Box::new(bybit_engine)],
        vec![Box::new(binance_collector), Box::new(bybit_collector)],
        vec![Box::new(echo_executor)],
        shutdown.clone(),
    );

    // Wait for shutdown signal
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");

    tracing::info!("Shutdown signal received, stopping bot...");
    shutdown.cancel();

    // Wait for all tasks to complete
    while let Some(result) = set.join_next().await {
        match result {
            Ok(_) => tracing::info!("Task completed successfully"),
            Err(e) => tracing::error!("Task failed: {}", e),
        }
    }
}
