use crate::client::CliClient;

use std::sync::Arc;

/// Execute config command.
pub async fn run_cmd(client: Arc<CliClient>) -> anyhow::Result<()> {
    let _channel = client.channel().await?;

    Ok(())
}
