use crate::GLOBAL_CONFIG_PATH;

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// Config filename trait.
pub trait ConfigMetadata {
    const FILE_NAME: &'static str;
}

/// Local config new type.
///
/// `Deref` is implemented.
/// `DerefMut` is implement.
#[derive(Debug, Default)]
pub struct LocalConfig<T>(pub T)
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
    T: ConfigMetadata,
    T: Default;

/// Implement general load/save functions for local config types.
impl<T> LocalConfig<T>
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
    T: ConfigMetadata,
    T: Default,
{
    /// Save the current config.
    pub async fn save(&self) -> Result<(), crate::error::Error> {
        tracing::info!("saving config: {}", T::FILE_NAME);

        // Serialize the current struct into a toml config file.
        let config_path = GLOBAL_CONFIG_PATH.join(T::FILE_NAME);
        let file_contents = toml::to_string_pretty(&self.0)?;

        tokio::fs::write(config_path, file_contents).await?;

        Ok(())
    }

    /// Load the current config from config path.
    pub async fn load() -> Result<Self, crate::error::Error> {
        tracing::info!("loading config: {}", T::FILE_NAME);

        let default = Self::default();
        let default_str = toml::to_string_pretty(&default.0)?;

        let config_path = GLOBAL_CONFIG_PATH.join(T::FILE_NAME);

        // Try load config starting with default values.
        let mut config = config::Config::builder();

        config = config.add_source(config::File::from_str(
            &default_str,
            config::FileFormat::Toml,
        ));

        // If our config exists, try applying other new values
        if config_path.exists() && config_path.is_file() {
            config = config.add_source(config::File::from(config_path));
        }

        let res = config
            .build()
            .map_err(|e| crate::error::Error::Config(e.to_string()))?
            .try_deserialize::<T>()
            .map_err(|e| crate::error::Error::Config(e.to_string()))?;

        Ok(Self(res))
    }
}

impl<T> Deref for LocalConfig<T>
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
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
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
    T: ConfigMetadata,
    T: Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
