use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// Create system tray for daemon.
fn system_tray(cancellation_token: CancellationToken) -> anyhow::Result<tray_item::TrayItem> {
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
    let task_tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    // Create system tray
    system_tray(cancellation_token.clone())?;

    let example_delay = tokio::time::Duration::from_secs_f64(1.0 / 10f64);
    let mut example_interval = tokio::time::interval(example_delay);

    // Start main program loop
    task_tracker.spawn({
        let task_tracker = task_tracker.clone();
        let cancellation_token = cancellation_token.clone();

        async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        break;
                    },
                    _ = example_interval.tick() => {

                    },
                }
            }
        }
    });

    // Wait for everything to finish before exiting
    task_tracker.close();
    task_tracker.wait().await;

    Ok(())
}
