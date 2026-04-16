use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{Layer as _Layer, Registry, filter, fmt::Layer, layer::SubscriberExt};

pub fn init_log() -> Vec<WorkerGuard> {
    let mut guards = vec![];

    let rolling_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(7)
        .filename_prefix("app")
        .filename_suffix("log")
        .build("./logs")
        .expect("initializing rolling file appender failed");
    let (rolling_writer, rolling_writer_guard) = tracing_appender::non_blocking(rolling_appender);
    guards.push(rolling_writer_guard);

    let file_logging_layer = BunyanFormattingLayer::new("app".into(), rolling_writer)
        .with_filter(filter::LevelFilter::INFO);
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(file_logging_layer);

    if cfg!(debug_assertions) {
        let (std_w, std_w_guard) = tracing_appender::non_blocking(std::io::stdout());
        let std_layer = Layer::new()
            .with_writer(std_w)
            .with_ansi(true)
            .with_file(true)
            .with_line_number(true)
            .with_filter(filter::LevelFilter::INFO);
        let subscriber = subscriber.with(std_layer);
        tracing::subscriber::set_global_default(subscriber)
            .expect("error setting global tracing subscriber");
        guards.push(std_w_guard);
        return guards;
    }

    tracing::subscriber::set_global_default(subscriber)
        .expect("error setting global tracing subscriber");
    guards
}
