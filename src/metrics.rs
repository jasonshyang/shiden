use metrics::{counter, describe_counter, describe_gauge, describe_histogram, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::{net::SocketAddr, time::Duration};
use tracing;

pub struct BotMetrics;

impl BotMetrics {
    pub fn init(addr: SocketAddr) {
        // Initialize metrics
        describe_counter!(
            "engine_events_processed_total",
            "Total number of events processed by each engine"
        );
        describe_histogram!(
            "engine_event_processing_seconds",
            "Time taken to process events by each engine"
        );
        describe_gauge!(
            "candle_series_size",
            "Size of the candle series for each engine"
        );
        describe_counter!(
            "component_errors_total",
            "Total number of errors encountered by each component"
        );

        // Start Prometheus exporter
        PrometheusBuilder::new()
            .with_http_listener(addr)
            .install()
            .expect("Failed to install Prometheus recorder");

        tracing::info!("Metrics server started on {}", addr);
    }

    pub fn record_event_processing(engine: &str, event_type: &str, duration: Duration) {
        counter!(
            "engine_events_processed_total",
            "engine" => engine.to_string(),
            "event_type" => event_type.to_string(),
        )
        .increment(1);

        histogram!(
            "engine_event_processing_seconds",
            "engine" => engine.to_string(),
            "event_type" => event_type.to_string()
        )
        .record(duration.as_secs_f64());
    }

    pub fn record_candle_series_size(engine: &str, size: usize) {
        gauge!(
            "candle_series_size",
            "engine" => engine.to_string(),
        )
        .set(size as u32);
    }

    pub fn record_error(component: &str) {
        counter!(
            "component_errors_total",
            "component" => component.to_string(),
        )
        .increment(1);
    }
}

pub struct DurationRecorder {
    start: std::time::Instant,
}

impl DurationRecorder {
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }

    pub fn end(self) -> Duration {
        self.start.elapsed()
    }
}
