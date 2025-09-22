use interprocess::local_socket;
use interprocess::local_socket::ToNsName;
use interprocess::local_socket::traits::tokio::Stream;
use tokio_util::codec::LengthDelimitedCodec;
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
    _cancellation_token: CancellationToken,
    stream: local_socket::tokio::Stream,
) -> anyhow::Result<()> {
    let stream_framed = LengthDelimitedCodec::builder().new_framed(stream);
    let channel_transport = tarpc::serde_transport::new(
        stream_framed,
        tarpc::tokio_serde::formats::Bincode::default(),
    );

    let context = tarpc::context::current();
    let client = shared_service::EchoClient::new(Default::default(), channel_transport).spawn();

    let result = client.echo(context, String::from("fuck")).await?;
    tracing::warn!("echo: {result}");

    let result = client.echo(context, String::from("shit")).await?;
    tracing::warn!("echo: {result}");

    let result = client.echo(context, String::from("dick")).await?;
    tracing::warn!("echo: {result}");

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
