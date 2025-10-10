use crate::client::CliClient;
use crate::error::Error;

use shared_service::client_client;
use std::sync::Arc;

/// Execute config command.
pub async fn run_cmd(client: Arc<CliClient>) -> Result<(), Error> {
    let channel = client.channel().await?;
    let mut client_service = client_client::ClientClient::new(channel);

    let response = client_service
        .info(tonic::Request::new(()))
        .await?
        .into_inner();

    client
        .terminal()
        .write_line(&format!("URL Api: {}", response.url_api))?;

    client
        .terminal()
        .write_line(&format!("URL Identity: {}", response.url_api))?;
    client
        .terminal()
        .write_line(&format!("Daemon uptime: {}s", response.uptime_seconds))?;

    Ok(())
}
