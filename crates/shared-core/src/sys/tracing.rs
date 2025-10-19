use std::path::Path;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::Layer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Install global tracing subscriber
pub fn init_tracing_subscriber(log_path: &Path) -> Result<(), crate::error::Error> {
    let filename = log_path.file_stem().unwrap().to_str().unwrap().to_string();
    let parent = log_path.parent().unwrap();

    // Create new writer which rolls logs every day.
    let writer_rolling_file = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(filename)
        .filename_suffix("log")
        .build(parent)
        .map_err(|e| crate::error::Error::LogFile(e.to_string()))?;

    // Only write to stdout if we are not the CLI client.
    // The CLI client uses a prettier console writer.
    let layer_stdout = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(std::io::stdout)
        .with_filter(LevelFilter::INFO);

    let layer_logfile = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(writer_rolling_file)
        .with_ansi(false)
        .with_filter(LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(layer_stdout)
        .with(layer_logfile)
        .try_init()
        .map_err(|e| crate::error::Error::TracingSubscriber(e.to_string()))?;

    Ok(())
}
