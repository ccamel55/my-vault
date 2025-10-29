mod client;
mod config;
mod constants;
mod controller;
mod middleware;
mod service;
mod view;

use crate::client::DaemonClient;
use crate::config::ConfigManager;

use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// My Vault daemon service.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address to server TCP connection.
    /// This can be either an ipv4 or ipv6 address.
    #[arg(
        short = 'a',
        long,
        default_value = "0.0.0.0:10001",
        env = "MY_VAULT_TCP_ADDRESS"
    )]
    tcp_address: String,

    /// Use TCP socket
    #[arg(short, long, env = "MY_VAULT_TCP_SOCKET")]
    tcp_socket: bool,
}

//noinspection DuplicatedCode
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared_core::sys::init_tracing_subscriber(&constants::GLOBAL_CACHE_PATH.join("daemon"))?;
    constants::create_global_paths().await?;

    let args = Args::parse();

    let task_tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    // Install signal handler to listen for cancellation.
    // This is extremely important as some things rely on dependable `Drop` invocation.
    let signal_handle = shared_core::sys::listen_for_cancellation(
        task_tracker.clone(),
        cancellation_token.clone(),
    )?;

    let config = Arc::new(ConfigManager::load().await?);
    let client = Arc::new(DaemonClient::start(config.clone()).await?);

    let close_fn;

    // Start serving our service
    if args.tcp_socket {
        let tcp_address = args.tcp_address;

        // Tcp requires no cleanup
        close_fn = None;

        let tcp = poem::listener::TcpListener::bind(&tcp_address);
        let app = service::create_services(false, config.clone(), client).await?;

        poem::Server::new(tcp)
            .run_with_graceful_shutdown(app, cancellation_token.cancelled(), None)
            .await?;
    } else {
        #[cfg(unix)]
        {
            let uds_socket_path = PathBuf::from("/tmp")
                .join(constants::FOLDER_NAME)
                .join("daemon.sock");

            tokio::fs::create_dir_all(&uds_socket_path.parent().unwrap()).await?;

            // Remove socket after using it otherwise we will error on startup
            // next time run the daemon.
            close_fn = Some({
                let uds_socket_path = uds_socket_path.clone();

                async move {
                    match tokio::fs::try_exists(&uds_socket_path).await {
                        Ok(exists) => {
                            tracing::debug!("does uds socket exist: {exists}");
                            if let Err(e) = tokio::fs::remove_file(&uds_socket_path).await {
                                tracing::warn!("could delete socket: {e}");
                            }
                        }
                        Err(e) => {
                            tracing::warn!(
                                "io error for socket path {} - {e}",
                                &uds_socket_path.display(),
                            )
                        }
                    };
                }
            });

            let uds = poem::listener::UnixListener::bind(&uds_socket_path);
            let app = service::create_services(true, config.clone(), client).await?;

            poem::Server::new(uds)
                .run_with_graceful_shutdown(app, cancellation_token.cancelled(), None)
                .await?;
        }

        #[cfg(not(unix))]
        {
            panic!("non unix platforms only support tcp")
        }
    }

    // Close signal stream
    signal_handle.close();

    // Wait for everything to finish before exiting
    task_tracker.close();
    task_tracker.wait().await;

    //
    // IMPORTANT: only do cleanup after task tracker has yield otherwise we might be
    //            removing resources still being used.
    //

    if let Err(e) = config.save().await {
        tracing::warn!("error saving config: {e}")
    }

    if let Some(close_fn) = close_fn {
        close_fn.await;
    }

    Ok(())
}
