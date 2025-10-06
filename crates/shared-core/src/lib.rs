pub mod config;
pub mod constants;
pub mod input;
pub mod signal;
pub mod tracing;

use std::path::PathBuf;

lazy_static::lazy_static! {
    /// Global config path.
    pub static ref GLOBAL_CONFIG_PATH: PathBuf = {
        dirs::config_local_dir()
            .expect("could not resolve config directory for current system")
            .join(constants::FOLDER_NAME)
    };

    /// Global cache path.
    pub static ref GLOBAL_CACHE_PATH: PathBuf = {
        dirs::cache_dir()
            .expect("could not resolve cache directory for current system")
            .join(constants::FOLDER_NAME)
    };
}

/// Local IPC socket name
pub fn local_socket_path() -> PathBuf {
    PathBuf::from("/tmp")
        .join("tonic")
        .join("bitwarden-rs-daemon.sock")
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
