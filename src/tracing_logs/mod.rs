use std::fs;
use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};

pub fn logger_start(service_name: &str, path: Option<String>) -> (WorkerGuard, WorkerGuard) {
    let log_dir = match path {
        Some(p) => format!("{}/logs", p),
        None => format!("logs"),
    };

    fs::create_dir_all(&log_dir).unwrap();

    let info_file = rolling::daily(&log_dir, format!("{}-info.log", service_name));
    let error_file = rolling::daily(&log_dir, format!("{}-error.log", service_name));

    let (info_writer, info_guard) = tracing_appender::non_blocking(info_file);
    let (error_writer, error_guard) = tracing_appender::non_blocking(error_file);

    let info_layer = fmt::layer()
        .with_writer(info_writer)
        .with_ansi(false)
        .with_level(true)
        .with_filter(filter_fn(|metadata| {
            metadata.level() == &tracing::Level::INFO
        }));

    let error_layer = fmt::layer()
        .with_writer(error_writer)
        .with_ansi(false)
        .with_level(true)
        .with_filter(LevelFilter::ERROR);

    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_level(true)
        .with_target(false) // 是否显示模块路径
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(info_layer)
        .with(error_layer)
        .with(console_layer)
        .init();

    (info_guard, error_guard)
}
