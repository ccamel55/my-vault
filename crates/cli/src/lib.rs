use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

pub mod cli;
pub mod config;
pub mod tui;

/// Name of folder for configs, logging, temp, etc.
const FOLDER_NAME: &str = "bitwarden-rs";

lazy_static! {
    /// Global config path.
    pub static ref GLOBAL_CONFIG_PATH: PathBuf = {
        dirs::config_local_dir()
            .expect("could not resolve config directory for current system")
            .join(FOLDER_NAME)
    };

    /// Global cache path.
    pub static ref GLOBAL_CACHE_PATH: PathBuf = {
        dirs::cache_dir()
            .expect("could not resolve cache directory for current system")
            .join(FOLDER_NAME)
    };
}

/// Global config states
pub struct GlobalConfigs {
    /// Client config
    pub client: Arc<RwLock<config::client::LocalClientConfig>>,
}

impl GlobalConfigs {
    /// Create a new global config instance.
    pub async fn load() -> anyhow::Result<Self> {
        let client = config::client::LocalClientConfig::load(GLOBAL_CONFIG_PATH.as_path(), true)
            .await
            .map(RwLock::new)
            .map(Arc::new)?;

        let result = Self { client };

        Ok(result)
    }
}

/// Install global tracing subscriber
pub fn init_tracing_subscriber() -> anyhow::Result<()> {
    // Default stdout is info, but this can change depending on env vars.
    let filter_stdout = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    // Create new writer which rolls logs every day.
    let writer_rolling_file = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("lib")
        .filename_suffix("log")
        .build(GLOBAL_CACHE_PATH.as_path())?;

    let layer_stdout = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(std::io::stdout)
        .and_then(filter_stdout);

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
