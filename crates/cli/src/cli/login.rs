use crate::GlobalConfigs;

use bitwarden_vault::VaultClientExt;
use clap::Subcommand;
use std::sync::Arc;

#[derive(Subcommand, Clone, Debug)]
pub enum LoginType {
    /// Login using API key
    Api {
        /// Client ID
        client_id: Option<String>,

        /// Client secret
        client_secret: Option<String>,
    },

    /// Login using password
    Password {
        /// Email
        email: Option<String>,
    },
}

/// Run login command
pub async fn run_login(
    _config: Arc<GlobalConfigs>,
    client: bitwarden_core::Client,
    login_type: LoginType,
) -> anyhow::Result<()> {
    match login_type {
        LoginType::Api {
            client_id,
            client_secret,
        } => {
            let client_id = crate::unwrap_or_prompt("Client ID", client_id)?;
            let client_secret = crate::unwrap_or_prompt("Client Secret", client_secret)?;

            let password = inquire::Password::new("Password")
                .without_confirmation()
                .prompt()?;

            // Create login request and try to send it to the server.
            let login_api_request = bitwarden_core::auth::login::ApiKeyLoginRequest {
                client_id,
                client_secret,
                password,
            };

            let result = client.auth().login_api_key(&login_api_request).await?;

            tracing::debug!("{result:?}");
        }
        LoginType::Password { email } => {
            let email = crate::unwrap_or_prompt("Email", email)?;
            let password = inquire::Password::new("Password")
                .without_confirmation()
                .prompt()?;

            let kdf = client.auth().prelogin(email.clone()).await?;

            let mut login_password_request = bitwarden_core::auth::login::PasswordLoginRequest {
                email,
                password,
                two_factor: None,
                kdf: kdf.clone(),
            };

            let result = client
                .auth()
                .login_password(&login_password_request)
                .await?;

            // Handle 2FA if we are prompted to do so
            if let Some(two_factor) = result.two_factor {
                if let Some(authenticator) = two_factor.authenticator {
                    tracing::debug!("{authenticator:?}");

                    let token = inquire::Text::new("Authenticator Code").prompt()?;
                    let two_factor_request = bitwarden_core::auth::login::TwoFactorRequest {
                        token,
                        provider: bitwarden_core::auth::login::TwoFactorProvider::Authenticator,
                        remember: false,
                    };

                    // Retry login again but with our 2FA
                    login_password_request.two_factor = Some(two_factor_request);

                    let result = client
                        .auth()
                        .login_password(&login_password_request)
                        .await?;

                    tracing::debug!("{result:?}");
                }
            } else {
                tracing::debug!("{result:?}");
            }
        }
    }

    // Todo:
    // - save the login details/tokens somewhere on the computer (make sure these are encrypted)
    // - use the saved login tokens to try automatically auth when CLI starts

    // TEST STUFF
    let sync_request = bitwarden_vault::SyncRequest {
        exclude_subdomains: None,
    };

    let sync_result = client.vault().sync(&sync_request).await?;

    tracing::info!("folders {}", sync_result.folders.len());
    tracing::info!("collections {}", sync_result.collections.len());
    tracing::info!("ciphers {}", sync_result.ciphers.len());

    Ok(())
}
