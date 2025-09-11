use lib::GLOBAL_CONFIG_PATH;
use lib::tui;
use lib::{cli, init_tracing_subscriber};

use anyhow::Result;
use clap::Parser;

mod constants {
    /// Default FPS
    pub const DEFAULT_FPS: u16 = 60;

    /// Default TPS
    pub const DEFAULT_TPS: u16 = 10;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Cli::parse();

    // TODO
    // - setup bitwarden SDK crate
    // - implement basic TUI

    let global_config_path = GLOBAL_CONFIG_PATH.to_path_buf();
    let global_config_path_exists = global_config_path.exists() && global_config_path.is_dir();

    init_tracing_subscriber()?;

    tracing::info!("config folder: {}", global_config_path.display());
    tracing::info!("creating config folder: {}", !global_config_path_exists);

    // Make sure that our config folder exists
    if !global_config_path_exists {
        tokio::fs::create_dir(&global_config_path).await?;
    }

    // Setup our global configs
    let configs = lib::GlobalConfigs::load().await?;

    {
        let fuck = configs.client.read().await;
        let poo = fuck.some_value_1.clone();

        tracing::info!("hello: {}", poo.unwrap_or("NONE".to_string()));
    }

    {
        let mut shit = configs.client.write().await;

        shit.some_value_1 = None;
    }

    {
        let fuck = configs.client.read().await;
        let poo = fuck.some_value_1.clone();

        tracing::info!("hello: {}", poo.unwrap_or("NONE".to_string()));
    }

    if let Some(sub_command) = args.sub_command {
        match sub_command {
            cli::SubCommands::Login { name } => Ok(()),
            cli::SubCommands::Logout { name } => Ok(()),
            cli::SubCommands::Select { name } => Ok(()),
        }
    } else {
        // Create TUI
        let mut tui = tui::Tui::start(constants::DEFAULT_FPS, constants::DEFAULT_TPS)?;

        // Shutdown TUI
        tui.wait().await
    }
}
