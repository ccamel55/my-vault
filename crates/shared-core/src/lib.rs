pub mod config;
mod constants;
pub mod input;
pub mod tracing;

use std::path::PathBuf;

/// Local IPC socket name
pub const LOCAL_SOCKET_NAME: &str = "bitwarden-rs-daemon.sock";

/// Name of folder for configs, logging, temp, etc.
const FOLDER_NAME: &str = "bitwarden-rs";

lazy_static::lazy_static! {
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

/// Client type
#[derive(PartialEq)]
pub enum Client {
    Cli,
    Daemon,
}

impl Client {
    /// Get client subfolder name
    pub fn sub_folder(&self) -> PathBuf {
        match self {
            Self::Cli => "cli",
            Self::Daemon => "daemon",
        }
        .into()
    }
}
