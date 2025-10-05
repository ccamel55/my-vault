mod client;
mod config;
mod database;
mod service;
mod system_tray;

use crate::client::DaemonClient;
use crate::system_tray::system_tray;

use futures::StreamExt;
use shared_core::local_socket_path;
use shared_service::{client_server, user_server};
use std::ffi::c_int;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tonic::codegen::tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

const SIGNALS: &[c_int] = &[
    signal_hook::consts::SIGINT,
    signal_hook::consts::SIGQUIT,
    signal_hook::consts::SIGTERM,
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared_core::tracing::init_subscriber(shared_core::Client::Daemon)?;

    let task_tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    // Add a signal handler to catch all cases where the program
    // will be shut down.
    let mut signal = signal_hook_tokio::Signals::new(SIGNALS)?;
    let signal_handle = signal.handle();

    task_tracker.spawn({
        let cancellation_token = cancellation_token.clone();
        async move {
            while let Some(signal) = signal.next().await {
                match signal {
                    signal_hook::consts::SIGINT
                    | signal_hook::consts::SIGQUIT
                    | signal_hook::consts::SIGTERM => {
                        // Invoke the cancellation token.
                        // This will start the normal shutdown process that all shutdowns do.
                        cancellation_token.cancel();
                    }
                    _ => unreachable!(),
                }
            }
        }
    });

    let client = Arc::new(
        DaemonClient::start(
            config::ConfigsDaemon::load().await?,
            database::Database::load().await?,
        )
        .await?,
    );

    // Create system tray
    system_tray(cancellation_token.clone())?;

    // Create UDS socket for accepting messages from client.s
    let uds_socket_path = local_socket_path();
    tokio::fs::create_dir_all(&uds_socket_path.parent().unwrap()).await?;

    tracing::info!("uds socket: {}", &uds_socket_path.display());

    let uds = UnixListener::bind(&uds_socket_path)?;
    let stream = UnixListenerStream::new(uds);

    // Create actual RPC services
    // TODO: create health check service
    let service_echo = service::ClientService::new(client.clone())?;
    let service_user = service::UserService::new(client.clone())?;

    Server::builder()
        .add_service(client_server::ClientServer::new(service_echo))
        .add_service(user_server::UserServer::new(service_user))
        .serve_with_incoming_shutdown(stream, cancellation_token.cancelled())
        .await?;

    // Close signal stream
    signal_handle.close();

    // Wait for everything to finish before exiting
    task_tracker.close();
    task_tracker.wait().await;

    //
    // IMPORTANT: only do cleanup after task tracker has yield otherwise we might be
    //            removing resources still being used.
    //

    // Remove socket after using it otherwise we will error on startup
    // next time run the daemon.
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
    }

    // Try to write back client data before exiting.
    if let Err(e) = client.try_save().await {
        tracing::warn!("could not save client data: {e}");
    }

    Ok(())
}
