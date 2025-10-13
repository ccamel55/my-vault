pub mod constants;
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
        .join("my-vault-daemon.sock")
}
