mod client;
mod database;
mod error;
mod middleware;
mod service;
mod system_tray;
mod view;

use crate::client::DaemonClient;
use crate::system_tray::system_tray;

use clap::Parser;
use shared_core::local_socket_path;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// My Vault daemon service.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address to server TCP connection.
    /// If none and on a supported platform, a socket will be used instead.
    #[arg(short, long, default_value = Some("0.0.0.0:10001".into()))]
    tcp_address: Option<String>,
}

//noinspection DuplicatedCode
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared_core::tracing::init_subscriber()?;
    shared_core::create_global_paths().await?;

    let args = Args::parse();

    let task_tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    // Install signal handler to listen for cancellation.
    // This is extremely important as some things rely on dependable `Drop` invocation.
    let signal_handle = shared_core::signal::listen_for_cancellation(
        task_tracker.clone(),
        cancellation_token.clone(),
    )?;

    let client = Arc::new(DaemonClient::start(database::Database::load().await?).await?);

    system_tray(cancellation_token.clone())?;

    let close_fn;

    // Start serving our service
    match args.tcp_address {
        Some(tcp_address) => {
            // Tcp requires no cleanup
            close_fn = None;

            // Create tcp listener stream
            tracing::info!("tcp address: {}", &tcp_address);

            let uds = tokio::net::TcpListener::bind(&tcp_address).await?;
            let stream = tonic::codegen::tokio_stream::wrappers::TcpListenerStream::new(uds);

            tonic::transport::Server::builder()
                .add_routes(service::create_services(client).await?)
                .serve_with_incoming_shutdown(stream, cancellation_token.cancelled())
                .await?;
        }
        None => {
            #[cfg(unix)]
            {
                let uds_socket_path = local_socket_path();
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

                // Create unix listener stream
                tracing::info!("uds socket: {}", &uds_socket_path.display());

                let uds = tokio::net::UnixListener::bind(&uds_socket_path)?;
                let stream = tonic::codegen::tokio_stream::wrappers::UnixListenerStream::new(uds);

                tonic::transport::Server::builder()
                    .add_routes(service::create_services(client).await?)
                    .serve_with_incoming_shutdown(stream, cancellation_token.cancelled())
                    .await?;
            }

            #[cfg(not(unix))]
            {
                panic!("non unix platforms only support tcp")
            }
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

    if let Some(close_fn) = close_fn {
        close_fn.await;
    }

    Ok(())
}
