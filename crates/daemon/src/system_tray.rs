use tokio_util::sync::CancellationToken;

/// Create system tray for daemon.
pub fn system_tray(cancellation_token: CancellationToken) -> anyhow::Result<tray_item::TrayItem> {
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
