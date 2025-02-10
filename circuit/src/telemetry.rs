use opentelemetry::global;
use opentelemetry::sdk::trace::{self, RandomIdGenerator, Sampler};
use opentelemetry::sdk::Resource;
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{
    fmt, prelude::*, registry::Registry, EnvFilter
};
use std::env;

pub fn init() {
    let endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".to_string());

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&endpoint),
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", "zk-proof-api"),
                    opentelemetry::KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                ]))
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("Failed to install OpenTelemetry tracer");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true);

    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env()
            .add_directive("api=debug".parse().unwrap())
            .add_directive("warn".parse().unwrap()))
        .with(telemetry)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
} 