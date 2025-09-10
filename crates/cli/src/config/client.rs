use crate::config::{ConfigMetadata, LocalConfig};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[allow(private_interfaces)]
pub type LocalClientConfig = LocalConfig<ClientConfig>;

/// Top level client settings
#[derive(Clone, Deserialize, Serialize)]
pub struct ClientConfig {
    pub some_value_1: Option<String>,
    pub some_value_2: Option<Vec<f32>>,
    pub some_value_3: Option<Vec<ExampleSubstruct>>,
}

/// Example of a substruct
#[derive(Clone, Deserialize, Serialize)]
pub struct ExampleSubstruct {
    pub name: String,
    pub some_property: i32,
}

impl ConfigMetadata for ClientConfig {
    fn filename() -> &'static str {
        "client.toml"
    }

    fn relative_path() -> PathBuf {
        PathBuf::new().join(Self::filename())
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            some_value_1: Some("hello i'm allan lol".into()),
            some_value_2: Some(vec![0.0, 0.1, 0.2, 0.3, 0.4]),
            some_value_3: Some(vec![
                ExampleSubstruct::default(),
                ExampleSubstruct::default(),
            ]),
        }
    }
}

impl Default for ExampleSubstruct {
    fn default() -> Self {
        Self {
            name: "fuck".into(),
            some_property: 69,
        }
    }
}
