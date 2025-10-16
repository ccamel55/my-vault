pub mod constants;
pub mod crypt;
pub mod database;
pub mod error;
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

/// Create folders if they don't exist yet
pub async fn create_global_paths() -> Result<(), tokio::io::Error> {
    let global_config_path = crate::GLOBAL_CONFIG_PATH.to_path_buf();
    let global_config_path_exists = global_config_path.exists() && global_config_path.is_dir();

    ::tracing::debug!("global config folder: {}", global_config_path.display());
    ::tracing::debug!(
        "creating global config folder: {}",
        !global_config_path_exists
    );

    if !global_config_path_exists {
        tokio::fs::create_dir(&global_config_path).await?;
    }

    let global_cache_path = crate::GLOBAL_CONFIG_PATH.to_path_buf();
    let global_cache_path_exists = global_cache_path.exists() && global_cache_path.is_dir();

    ::tracing::debug!("global cache folder: {}", global_cache_path.display());
    ::tracing::debug!(
        "creating global cache folder: {}",
        !global_cache_path_exists
    );

    if !global_cache_path_exists {
        tokio::fs::create_dir(&global_cache_path).await?;
    }

    Ok(())
}
