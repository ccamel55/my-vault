use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::path::Path;

/// Local config new type.
///
/// `Deref` is implemented.
/// `DerefMut` is implement.
#[derive(Debug, Default)]
pub struct LocalConfig<T>(pub T)
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
    T: Default;

/// Implement general load/save functions for local config types.
impl<T> LocalConfig<T>
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
    T: Default,
{
    /// Save the current config.
    pub async fn save(&self, config_path: &Path) -> Result<(), crate::error::Error> {
        tracing::info!("saving config: {}", &config_path.display());

        // Serialize the current struct into a toml config file.
        let file_contents = toml::to_string_pretty(&self.0)?;

        tokio::fs::write(config_path, file_contents).await?;

        Ok(())
    }

    /// Load the current config from config path.
    pub async fn load(config_path: &Path) -> Result<Self, crate::error::Error> {
        tracing::info!("loading config: {}", &config_path.display());

        let default = Self::default();
        let default_str = toml::to_string_pretty(&default.0)?;

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
    T: Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::rng;
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// Test config
    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct Config {
        pub some_value_1: usize,
        pub some_value_2: Vec<usize>,
        pub some_value_3: HashMap<String, usize>,
        pub some_value_4: SubConfig,
        pub some_value_5: Vec<SubConfig>,
        pub some_value_6: HashMap<String, SubConfig>,
    }

    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize, PartialEq)]
    pub struct SubConfig {
        pub value_1: String,
        pub value_2: f64,
    }

    impl SubConfig {
        pub fn new(index: usize) -> Self {
            Self {
                value_1: format!("my index is {}", index),
                value_2: index as f64 / 3.33,
            }
        }
    }

    pub type LocalConfig = crate::config::LocalConfig<Config>;

    #[tokio::test]
    async fn save_load() {
        // Add shit to out config
        let mut config = LocalConfig::default();

        config.some_value_1 = 69;
        config.some_value_2 = (0..10).collect();
        config.some_value_3 = (0..5)
            .map(|x| (format!("map-primitive-{}", x), x))
            .collect();

        config.some_value_4 = SubConfig::new(69);
        config.some_value_5 = (0..10).map(|x| SubConfig::new(x)).collect();
        config.some_value_6 = (0..5)
            .map(|x| (format!("map-primitive-{}", x), SubConfig::new(x)))
            .collect();

        // Create random file in test data folder
        let config_path = PathBuf::from(env!("WORKSPACE_DIR"))
            .join("test-data")
            .join("temp")
            .join(format!("{}.toml", rng::random_string(10)));

        let result_save = config.save(&config_path).await;
        let result_load = LocalConfig::load(&config_path).await;

        assert!(result_save.is_ok());
        assert!(result_load.is_ok());

        assert!(tokio::fs::remove_file(&config_path).await.is_ok());

        // Make sure all our data is the same
        let config_2 = result_load.unwrap();

        assert_eq!(config_2.some_value_1, config.some_value_1);
        assert_eq!(config_2.some_value_2, config.some_value_2);
        assert_eq!(config_2.some_value_3, config.some_value_3);
        assert_eq!(config_2.some_value_4, config.some_value_4);
        assert_eq!(config_2.some_value_5, config.some_value_5);
        assert_eq!(config_2.some_value_6, config.some_value_6);
    }

    #[tokio::test]
    async fn load_empty() {
        let config_path = PathBuf::from(env!("WORKSPACE_DIR"))
            .join("test-data")
            .join("config")
            .join("empty.toml");

        let result = LocalConfig::load(&config_path).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn load_unknown_fields() {
        let config_path = PathBuf::from(env!("WORKSPACE_DIR"))
            .join("test-data")
            .join("config")
            .join("unknown-fields.toml");

        let result = LocalConfig::load(&config_path).await;

        assert!(result.is_ok());
    }
}
