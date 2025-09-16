use interprocess::local_socket;
use interprocess::local_socket::ToNsName;
use interprocess::local_socket::traits::tokio::Listener;
use interprocess::local_socket::traits::tokio::Stream;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

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

/// Setup socket listener
async fn socket_listener() -> anyhow::Result<local_socket::tokio::Listener> {
    tracing::info!(
        "creating socket listener: {}",
        shared_core::LOCAL_SOCKET_NAME
    );

    let socket_name =
        shared_core::LOCAL_SOCKET_NAME.to_ns_name::<local_socket::GenericNamespaced>()?;

    let listener = local_socket::ListenerOptions::new()
        .name(socket_name)
        .reclaim_name(true)
        .create_tokio();

    let listener = match listener {
        Ok(x) => x,
        Err(e) => {
            if e.kind() == io::ErrorKind::AddrInUse {
                tracing::error!("Could not start socket because socket file already exists.");
            }
            return Err(e.into());
        }
    };

    Ok(listener)
}

/// Socker connection handler
async fn connection_handler(
    _task_tracker: TaskTracker,
    _cancellation_token: CancellationToken,
    stream: local_socket::tokio::Stream,
) -> anyhow::Result<()> {
    tracing::info!("connection opened");

    let (recv, mut send) = stream.split();
    let mut recv = BufReader::new(recv);

    // Temporary write buffer
    // Note: must not clear this buffer as read only works on a sized/initialized buffer.
    let mut buffer = vec![0; shared_core::LOCAL_SOCKET_BUFFER_SIZE];

    loop {
        // Wait for message
        let length = match recv.read(&mut buffer).await {
            Ok(0) => break,
            Ok(x) => x,
            Err(e) => {
                tracing::error!("could not receive data: {}", e);
                break;
            }
        };

        let buffer_string = str::from_utf8(&buffer[0..length])?;
        tracing::info!("received ({}) - {}", length, buffer_string);

        // Wait a second before doing anything
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Send that shit straight back because y the fuck not
        send.write_all(b"FUCK").await?;
    }

    tracing::info!("connection closed");

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Temporary tracing subscriber
    // TODO: replace with propper logging subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let task_tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    // Create system tray
    system_tray(cancellation_token.clone())?;

    tokio::select! {
        _ = cancellation_token.cancelled() => {
            // NOTHING - shutdown was invoked
        },
        result = async {
            // Create unnamed socket listener
            let listener = socket_listener().await?;

            loop {
                // Handle any errors with socket connection
                let stream = listener.accept().await?;

                // Handle new connection
                // TODO: limit this to only one active connection at a time
                task_tracker.spawn({
                    let task_tracker = task_tracker.clone();
                    let cancellation_token = cancellation_token.child_token();

                    async move {
                        if let Err(e) = connection_handler(
                            task_tracker,
                            cancellation_token,
                            stream,
                        ).await {
                            tracing::error!("could not handle connection: {}", e.to_string())
                        }
                    }
                });
            }

            // Help infer return type
            #[allow(unreachable_code)]
            Ok::<_, anyhow::Error>(())
        } => {
            result?;
        },
    }

    // Wait for everything to finish before exiting
    task_tracker.close();
    task_tracker.wait().await;

    Ok(())
}
