use inquire::error::InquireResult;
use lazy_static::lazy_static;
use std::path::PathBuf;
use tokio::sync::RwLock;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::Layer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
    pub client: RwLock<config::client::LocalClientConfig>,

    /// User config
    pub user: RwLock<config::user::LocalUserConfig>,
}

impl GlobalConfigs {
    /// Create a new global config instance.
    pub async fn load() -> anyhow::Result<Self> {
        let global_config_path = GLOBAL_CONFIG_PATH.to_path_buf();
        let global_config_path_exists = global_config_path.exists() && global_config_path.is_dir();

        tracing::debug!("config folder: {}", global_config_path.display());
        tracing::debug!("creating config folder: {}", !global_config_path_exists);

        // Make sure that our config folder exists
        if !global_config_path_exists {
            tokio::fs::create_dir(&global_config_path).await?;
        }

        let client = config::client::LocalClientConfig::load(&global_config_path, true)
            .await
            .map(RwLock::new)?;

        let user = config::user::LocalUserConfig::load(&global_config_path, true)
            .await
            .map(RwLock::new)?;

        let result = Self { client, user };

        Ok(result)
    }

    /// Attempt to do a save using `try_read`.
    pub async fn try_save(&self) -> anyhow::Result<()> {
        if let Ok(config) = self.client.try_read() {
            config.save(GLOBAL_CONFIG_PATH.as_path()).await?;
        }

        if let Ok(config) = self.user.try_read() {
            config.save(GLOBAL_CONFIG_PATH.as_path()).await?;
        }

        Ok(())
    }
}

/// Install global tracing subscriber
pub fn init_tracing_subscriber() -> anyhow::Result<()> {
    // Create new writer which rolls logs every day.
    let writer_rolling_file = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("lib")
        .filename_suffix("log")
        .build(GLOBAL_CACHE_PATH.as_path())?;

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


