use crate::GLOBAL_CACHE_PATH;

use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::Layer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Install global tracing subscriber
pub fn init_subscriber() -> anyhow::Result<()> {
    // Create new writer which rolls logs every day.
    let writer_rolling_file = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("lib")
        .filename_suffix("log")
        .build(GLOBAL_CACHE_PATH.as_path().join("daemon"))?;

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
        .try_init()?;

    Ok(())
}
