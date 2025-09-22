use futures::prelude::*;
use interprocess::local_socket;
use interprocess::local_socket::ToNsName;
use interprocess::local_socket::traits::tokio::Listener;
use shared_core::service::Echo;
use tarpc::context::Context;
use tarpc::server::Channel;
use tokio::io;
use tokio_util::codec::LengthDelimitedCodec;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

#[derive(Clone)]
struct EchoService;

impl Echo for EchoService {
    async fn health_check(self, _context: Context) -> () {
        // EMPTY
    }

    async fn echo(self, _context: Context, name: String) -> String {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        format!("hello {name}")
    }
}

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
    let stream_framed = LengthDelimitedCodec::builder().new_framed(stream);
    let channel_transport = tarpc::serde_transport::new(
        stream_framed,
        tarpc::tokio_serde::formats::Bincode::default(),
    );

    tarpc::server::BaseChannel::with_defaults(channel_transport)
        .execute(EchoService.serve())
        .for_each(|x| x)
        .await;

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
