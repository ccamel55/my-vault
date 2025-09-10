use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt};

pub mod cli;
pub mod config;
pub mod tui;

/// Name of folder for configs, logging, temp, etc.
const FOLDER_NAME: &str = "bitwarden-rs";

lazy_static! {
    /// Global config path
    pub static ref GLOBAL_CONFIG_PATH: PathBuf = {
        dirs::config_local_dir()
            .expect("could not resolve config directory for current system")
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
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
