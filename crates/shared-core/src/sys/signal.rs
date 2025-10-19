use futures::StreamExt;
use std::ffi::c_int;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// Signals we are listening for
const SIGNALS: &[c_int] = &[
    signal_hook::consts::SIGINT,
    signal_hook::consts::SIGQUIT,
    signal_hook::consts::SIGTERM,
];

/// Add a signal handler to catch all cases where the program
// will be shut down.
pub fn listen_for_cancellation(
    task_tracker: TaskTracker,
    cancellation_token: CancellationToken,
) -> Result<signal_hook::iterator::Handle, std::io::Error> {
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

    Ok(signal_handle)
}
