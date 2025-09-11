use serde::Serialize;
use serde::de::DeserializeOwned;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub mod client;

/// Config filename trait.
pub trait ConfigMetadata {
    /// Get the filename of a config.
    fn filename() -> &'static str;

    /// Get the relative path of a config from the global config directory.
    fn relative_path() -> PathBuf;
}

/// Local config new type.
///
/// `Default` is implemented.
/// `Deref` is implemented.
/// `DerefMut` is implement.
pub struct LocalConfig<T>(pub T)
where
    T: DeserializeOwned + Serialize,
    T: ConfigMetadata,
    T: Default;

/// Implement general load/save functions for local config types.
impl<T> LocalConfig<T>
where
    T: DeserializeOwned + Serialize,
    T: ConfigMetadata,
    T: Default,
{
    /// Save the current config.
    pub async fn save(&self, global_config_path: &Path) -> anyhow::Result<()> {
        tracing::debug!("saving config: {}", T::filename());

        if !global_config_path.exists() || !global_config_path.is_dir() {
            return Err(anyhow::anyhow!(
                "global config path does not exist: {}",
                global_config_path.display()
            ));
        }

        // Serialize the current struct into a toml config file.
        let config_path = global_config_path.join(T::relative_path());
        let file_contents = toml::to_string_pretty(&self.0)?;

        tokio::fs::write(config_path, file_contents).await?;

        Ok(())
    }

    /// Load the current config from config path.
    pub async fn load(global_config_path: &Path, create_if_missing: bool) -> anyhow::Result<Self> {
        tracing::debug!("loading config: {}", T::filename());

        if !global_config_path.exists() || !global_config_path.is_dir() {
            return Err(anyhow::anyhow!(
                "global config path does not exist: {}",
                global_config_path.display()
            ));
        }

        let config_path = global_config_path.join(T::relative_path());

        if !config_path.exists() || !config_path.is_file() {
            tracing::debug!("config does not exist: {}", T::filename());
            tracing::debug!("creating default config: {}", create_if_missing);

            let default = Self::default();

            if create_if_missing {
                default.save(global_config_path).await?;
            }

            return Ok(default);
        }

        // Read config file from disk and deserialize it into the struct.
        let file_contents = tokio::fs::read_to_string(config_path).await?;
        let data: T = toml::from_str(&file_contents)
            .or(Err(anyhow::anyhow!("could not deserialize toml config")))?;

        Ok(Self(data))
    }
}

impl<T> Default for LocalConfig<T>
where
    T: DeserializeOwned + Serialize,
    T: ConfigMetadata,
    T: Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T> Deref for LocalConfig<T>
where
    T: DeserializeOwned + Serialize,
    T: ConfigMetadata,
    T: Default,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for LocalConfig<T>
where
    T: DeserializeOwned + Serialize,
    T: ConfigMetadata,
    T: Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
