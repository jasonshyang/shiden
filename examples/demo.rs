use shiden::{engines::price::PriceStateEngine, metrics::BotMetrics, run::run_bot};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    let metrics_addr: std::net::SocketAddr = "0.0.0.0:9090".parse().unwrap();
    BotMetrics::init(metrics_addr);

    // Initialize the Echo strategy
    let echo_strategy = shiden::strategies::echo::EchoStrategy;
    let binance_collector = shiden::collectors::binance::BinanceCollector;
    let bybit_collector = shiden::collectors::bybit::BybitCollector;
    let coinbase_collector = shiden::collectors::coinbase::CoinbaseCollector;
    let echo_executor = shiden::executors::echo::EchoExecutor;
    let price_engine = PriceStateEngine::new(1_000); // 60 second candle timeframe
    let shutdown = CancellationToken::new();

    let mut set = run_bot(
        echo_strategy,
        vec![Box::new(price_engine)],
        vec![
            Box::new(binance_collector),
            Box::new(bybit_collector),
            Box::new(coinbase_collector),
        ],
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
