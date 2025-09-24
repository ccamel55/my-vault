mod config;
mod service;

use shared_core::local_socket_path;
use shared_service::{echo_server, user_server};
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tonic::codegen::tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

/// Create system tray for daemon.
/// Todo: display connection info in system tray
fn system_tray(cancellation_token: CancellationToken) -> anyhow::Result<tray_item::TrayItem> {
    tracing::info!("creating system tray");

    let cursor = std::io::Cursor::new(include_bytes!("../../../resources/tray_icon.png"));
    let decoder = png::Decoder::new(cursor);

    let mut reader = decoder.read_info()?;
    let mut data = vec![0; reader.output_buffer_size().unwrap()];

    let _ = reader.next_frame(&mut data)?;
    let icon = tray_item::IconSource::Data {
        data,
        height: reader.info().height as i32,
        width: reader.info().width as i32,
    };

    let mut tray = tray_item::TrayItem::new("Bitwarden RS", icon)?;

    tray.add_label("Bitwarden RS daemon")?;

    tray.add_menu_item("Quit", move || {
        cancellation_token.cancel();
    })?;

    Ok(tray)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared_core::tracing::init_subscriber(shared_core::Client::Daemon)?;

    // Setup our global configs
    let configs = Arc::new(config::ConfigsDaemon::load().await?);

    let task_tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    // Create system tray
    system_tray(cancellation_token.clone())?;

    let uds_path = local_socket_path();
    tokio::fs::create_dir_all(uds_path.parent().unwrap()).await?;

    // Create socket listener
    let uds = UnixListener::bind(&uds_path)?;
    let stream = UnixListenerStream::new(uds);

    // Create service
    let service_echo = service::EchoService {};
    let service_user = service::UserService {};

    Server::builder()
        .add_service(echo_server::EchoServer::new(service_echo))
        .add_service(user_server::UserServer::new(service_user))
        .serve_with_incoming_shutdown(stream, cancellation_token.cancelled())
        .await?;

    // Remove socket after using it
    tokio::fs::remove_file(&uds_path).await?;

    // Wait for everything to finish before exiting
    task_tracker.close();
    task_tracker.wait().await;

    // Try save config before we exit.
    configs.try_save().await?;

    Ok(())
}
