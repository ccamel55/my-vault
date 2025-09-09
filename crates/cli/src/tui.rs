use crossterm::event::{Event, KeyCode, KeyModifiers};
use futures::FutureExt;
use futures::StreamExt;
use ratatui::layout::Alignment;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use std::sync::{Arc, Mutex};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// Text UI object
pub struct Tui {
    terminal: Arc<Mutex<ratatui::DefaultTerminal>>,
    task_tracker: TaskTracker,
}

impl Tui {
    /// Setup terminal properties and start ratatui.
    pub fn start(max_fps: u16, max_tps: u16) -> anyhow::Result<Self> {
        // Setup terminal
        let terminal = Arc::new(Mutex::new(ratatui::init()));

        // Create ratatui execution task
        let fps_delay = std::time::Duration::from_secs_f64(1.0 / max_fps as f64);
        let tps_delay = std::time::Duration::from_secs_f64(1.0 / max_tps as f64);

        let task_tracker = TaskTracker::new();
        let cancellation_token = CancellationToken::new();

        // Render
        task_tracker.spawn({
            let terminal = terminal.clone();
            let cancellation_token = cancellation_token.clone();

            async move {
                let mut delay_interval = tokio::time::interval(fps_delay);

                loop {
                    tokio::select! {
                        _ = cancellation_token.cancelled() => {
                            break;
                        },
                        _ = delay_interval.tick() => {
                            // Todo: handle error!!
                            let mut terminal = terminal.lock().unwrap();
                            let _ = terminal.draw(|frame| {
                               Self::draw_ui(frame)
                            });
                        },
                    }
                }
            }
        });

        // Update/tick
        task_tracker.spawn({
            let cancellation_token = cancellation_token.clone();

            async move {
                let mut delay_interval = tokio::time::interval(tps_delay);
                let mut event_stream = crossterm::event::EventStream::new();

                loop {
                    tokio::select! {
                        _ = cancellation_token.cancelled() => {
                            break;
                        },
                        _ = delay_interval.tick() => {

                        },
                        event = event_stream.next().fuse() => {
                            match event {
                                Some(Ok(event)) => {
                                    match event {
                                        Event::Key(event) => {
                                            // Check for exit keys
                                            if event.code == KeyCode::Esc || (event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c')) {
                                                cancellation_token.cancel();
                                            }
                                        },
                                        _ => {

                                        }
                                    }
                                },
                                Some(Err(_error)) => {
                                    // Todo: handle error!!
                                },
                                None => {

                                }
                            }
                        }
                    }
                }
            }
        });

        let instance = Self {
            terminal,
            task_tracker,
        };

        Ok(instance)
    }

    /// Wait for all tasks to finish
    pub async fn wait(&mut self) -> anyhow::Result<()> {
        // Must be closed before we can await on it.
        // Note: nothing prevents us from spawning new tasks after closing. Be careful!
        self.task_tracker.close();
        self.task_tracker.wait().await;

        self.terminal.lock().unwrap().flush()?;

        // Restore terminal state
        ratatui::restore();

        Ok(())
    }

    /// UI for terminal
    fn draw_ui(frame: &mut ratatui::Frame) {
        let area = frame.area();
        frame.render_widget(
            Paragraph::new(format!(
                "Press j or k to increment or decrement.\n\nCounter: {}",
                69,
            ))
            .block(
                Block::default()
                    .title("ratatui async counter app")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center),
            area,
        );
    }
}
