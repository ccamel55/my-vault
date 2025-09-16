use interprocess::local_socket;
use interprocess::local_socket::ToNsName;
use interprocess::local_socket::traits::tokio::Stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// Create socket stream
async fn socket_stream() -> anyhow::Result<local_socket::tokio::Stream> {
    tracing::info!("creating socket stream: {}", shared_core::LOCAL_SOCKET_NAME);

    let socket_name =
        shared_core::LOCAL_SOCKET_NAME.to_ns_name::<local_socket::GenericNamespaced>()?;

    local_socket::tokio::Stream::connect(socket_name)
        .await
        .map_err(anyhow::Error::from)
}

/// Connection handler
async fn connection_handler(
    _task_tracker: TaskTracker,
    cancellation_token: CancellationToken,
    stream: local_socket::tokio::Stream,
) -> anyhow::Result<()> {
    tracing::info!("connection opened");

    let (recv, mut send) = stream.split();
    let mut recv = BufReader::new(recv);

    // Temporary write buffer
    // Note: must not clear this buffer as read only works on a sized/initialized buffer.
    let mut buffer = vec![0; shared_core::LOCAL_SOCKET_BUFFER_SIZE];

    loop {
        if cancellation_token.is_cancelled() {
            break;
        }

        // Wait a second before doing anything
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Send data to the listener
        if let Err(e) = send.write_all(b"FUCK SHIT BALLS").await {
            tracing::error!("could not send data: {}", e);
            break;
        }

        // Wait for data to come back
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

    // Create socket stream
    let stream = socket_stream().await?;

    // Setup listener for ctrl-c to gracefully shutdown
    tokio::task::spawn({
        let cancellation_token = cancellation_token.clone();

        async move {
            tokio::signal::ctrl_c()
                .await
                .expect("could not register signal");

            cancellation_token.cancel();
        }
    });

    // Task responsible for handling socket communication
    task_tracker.spawn(connection_handler(
        task_tracker.clone(),
        cancellation_token.child_token(),
        stream,
    ));

    // Wait for everything to finish before exiting
    task_tracker.close();
    task_tracker.wait().await;

    Ok(())
}
