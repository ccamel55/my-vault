use crate::GlobalConfigs;

use bitwarden_vault::VaultClientExt;
use std::sync::Arc;

/// Run login command
pub async fn run_login_new(
    config: Arc<GlobalConfigs>,
    client: bitwarden_core::Client,
) -> anyhow::Result<()> {
    let alias = inquire::Text::new("Alias").prompt()?;
    if config.user.read().await.users.contains_key(&alias) {
        return Err(anyhow::anyhow!(
            "user with alias '{}' already exists",
            &alias
        ));
    }

    let email = inquire::Text::new("Email").prompt()?;
    let password = inquire::Password::new("Password")
        .without_confirmation()
        .prompt()?;

    let kdf = client.auth().prelogin(email.clone()).await?;

    let mut login_password_request = bitwarden_core::auth::login::PasswordLoginRequest {
        email: email.clone(),
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
                remember: true,
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

    // Add our device to trusted devices and save the
    let trust_result = client.auth().trust_device()?;
    let user = crate::config::user::UserEntryConfig {
        user_id: client.internal.get_user_id(),
        email,
        kdf,
        device_key: trust_result.device_key.into(),
        user_key: trust_result.protected_user_key,
        device_public_key: trust_result.protected_device_public_key,
        device_private_key: trust_result.protected_device_private_key,
    };

    // Write back changes to file
    {
        config.user.write().await.users.insert(alias, user);
        config.try_save().await?;
    }

    // Todo:
    // - save the login details/tokens somewhere on the computer (make sure these are encrypted)
    // - use the saved login tokens to try automatically auth when CLI starts
    // - test_device_login() in bitwarden_core for example of relogin

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

pub async fn run_login_alias(
    config: Arc<GlobalConfigs>,
    client: bitwarden_core::Client,
    alias: Option<String>,
) -> anyhow::Result<()> {
    let alias = inquire::Text::new("Alias").prompt()?;
    if !config.user.read().await.users.contains_key(&alias) {
        return Err(anyhow::anyhow!(
            "user with alias '{}' does not exists",
            &alias
        ));
    }

    Ok(())
}
