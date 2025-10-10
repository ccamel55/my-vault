use crate::client::CliClient;
use crate::error::Error;

use shared_service::client_client;
use std::sync::Arc;

#[derive(clap::Args, Clone, Debug)]
pub struct Args {
    /// Url of api endpoint
    #[clap(long, default_value = None, required = false)]
    url_api: Option<String>,

    /// Url of identity endpoint
    #[clap(long, default_value = None, required = false)]
    url_identity: Option<String>,
}

impl Args {
    /// Checks if arguments are all empty
    pub fn is_empty(&self) -> bool {
        self.url_api.is_none() && self.url_api.is_none()
    }
}

/// Execute config command.
pub async fn run_cmd(client: Arc<CliClient>, args: &Args) -> Result<(), Error> {
    if args.is_empty() {
        return Err(Error::Argument(
            "no values provided to config command".into(),
        ));
    }

    let channel = client.channel().await?;
    let mut client_service = client_client::ClientClient::new(channel);

    Ok(())
}
