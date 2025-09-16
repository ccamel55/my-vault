use lib::tui;
use lib::{cli, init_tracing_subscriber};

use clap::Parser;
use std::sync::Arc;
use url::Url;

mod constants {
    /// Default FPS
    pub const DEFAULT_FPS: u16 = 60;

    /// Default TPS
    pub const DEFAULT_TPS: u16 = 10;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    // TODO
    // - setup bitwarden SDK crate
    // - implement basic TUI
    // - make sure cancellation works correctly.

    init_tracing_subscriber()?;

    // Setup our global configs
    let configs = Arc::new(lib::GlobalConfigs::load().await?);

    // Update client URL if needed
    if let Some(base_url) = args.server {
        Url::parse(&base_url)?;

        {
            let mut config_client = configs.client.write().await;

            config_client.connection.url_api = format!("{base_url}/api");
            config_client.connection.url_identity = format!("{base_url}/identity");
        }

        configs.try_save().await?;
    }

    // Create bitwarden client
    let client = {
        let config_client = configs.client.read().await.clone();

        tracing::debug!("url api: {}", &config_client.connection.url_api);
        tracing::debug!("url identity: {}", &config_client.connection.url_api);

        let client_settings = bitwarden_core::ClientSettings {
            api_url: config_client.connection.url_api,
            identity_url: config_client.connection.url_identity,
            user_agent: "Bitwarden CLI Rust".into(),
            device_type: bitwarden_core::DeviceType::SDK,
        };

        bitwarden_core::Client::new(Some(client_settings))
    };

    if let Some(command) = args.command {
        command.run(configs.clone(), client).await?;
    } else {
        // Create TUI
        let mut tui = tui::Tui::start(constants::DEFAULT_FPS, constants::DEFAULT_TPS)?;

        // Shutdown TUI
        tui.wait().await?;
    };

    Ok(())
}
