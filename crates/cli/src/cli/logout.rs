use crate::GlobalConfigs;
use std::sync::Arc;

pub async fn run_logout(
    config: Arc<GlobalConfigs>,
    client: bitwarden_core::Client,
    alias: Option<String>,
) -> anyhow::Result<()> {
    let alias = inquire::Text::new("Alias").prompt()?;

    Ok(())
}
