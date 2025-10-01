mod command;

use clap::Parser;

/// Bitwarden CLI crab edition
#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: command::Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared_core::tracing::init_subscriber(shared_core::Client::Cli)?;

    let args = Cli::parse();

    // let task_tracker = TaskTracker::new();
    // let cancellation_token = CancellationToken::new();
    //
    // // Create socket stream
    // // Note: Uri is ignored because we are using our own connector
    // let channel = Endpoint::try_from("http://[::]:50051")?
    //     .connect_with_connector(service_fn(|_: Uri| async {
    //         let uds_path = local_socket_path();
    //         let stream = UnixStream::connect(uds_path).await?;
    //
    //         Ok::<_, std::io::Error>(TokioIo::new(stream))
    //     }))
    //     .await?;
    //
    // let request = EchoRequest {
    //     message: "fuck".into(),
    // };
    //
    // let mut client = echo_client::EchoClient::new(channel);
    //
    // task_tracker.spawn({
    //     let cancellation_token = cancellation_token.clone();
    //
    //     async move {
    //         select! {
    //             _ = cancellation_token.cancelled() => {
    //
    //             },
    //             response = client.echo(tonic::Request::new(request)) => {
    //                 let response = response?.into_inner();
    //                 tracing::info!("response: {}", response.message);
    //             }
    //         }
    //
    //         // Help infer return type
    //         Ok::<_, anyhow::Error>(())
    //     }
    // });

    // Execute what ever command is being passed.
    args.command.run().await?;

    // Wait for everything to finish before exiting
    // task_tracker.close();
    // task_tracker.wait().await;

    Ok(())
}
