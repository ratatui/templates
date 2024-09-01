use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Terminal events.
#[derive(Debug)]
pub enum Event {
    /// The tick event is sent at a regular interval and can be used to trigger application state
    /// updates or animations.
    Tick,

    /// Key press.
    Key(KeyEvent),

    /// Mouse click/scroll.
    Mouse(MouseEvent),

    /// Terminal resize.
    ///
    /// This event may be fired multiple times in quick succession, so it is recommended not to
    /// render the UI on every resize event.
    Resize(u16, u16),

    /// An error occurred.
    Error(std::io::Error),
}

/// Terminal event handler.
///
/// This struct is responsible for listening to terminal events and sending them to the main thread.
///
/// It sends a tick event at a regular interval. The tick rate is specified by the `tick_rate`
/// parameter. If an error occurs, it sends an error event to the main thread and then stops
/// running.
#[derive(Debug)]
pub struct EventSource {
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
}

impl EventSource {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || event_thread(sender, tick_rate));
        Self { receiver }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if there is no data available unless the
    /// event source has been closed.
    pub fn next(&self) -> color_eyre::Result<Event> {
        let event = self.receiver.recv()?;
        Ok(event)
    }
}

/// An event thread that listens for terminal events.
///
/// This function is responsible for listening to terminal events and sending them to the main
/// thread. It sends a tick event at a regular interval. The tick rate is specified by the
/// `tick_rate` parameter. If an error occurs, it sends an error event to the main thread and then
/// stops running.
///
/// Errors sending an event are ignored as this generally indicates that the main thread has exited
/// and is no longer listening for events.
fn event_thread(sender: mpsc::Sender<Event>, tick_rate: Duration) {
    let mut last_tick: Option<Instant> = None;
    loop {
        if last_tick
            .map(|tick| tick.elapsed() >= tick_rate)
            .unwrap_or(true)
        {
            let _ = sender.send(Event::Tick);
            last_tick = Some(Instant::now());
        }

        let timeout = last_tick.map_or(Duration::ZERO, |tick| {
            tick_rate.saturating_sub(tick.elapsed())
        });

        match event::poll(timeout) {
            Ok(false) => {
                // no new events waiting
                continue;
            }
            Ok(true) => {}
            Err(e) => {
                let _ = sender.send(Event::Error(e));
                break;
            }
        }

        match event::read() {
            Err(err) => {
                let _ = sender.send(Event::Error(err));
                break;
            }
            Ok(event) => match event {
                CrosstermEvent::Key(e) if e.kind == KeyEventKind::Press => {
                    // ignore key release / repeat events
                    let _ = sender.send(Event::Key(e));
                }
                CrosstermEvent::Key(_) => {}
                CrosstermEvent::Mouse(e) => {
                    let _ = sender.send(Event::Mouse(e));
                }
                CrosstermEvent::Resize(w, h) => {
                    let _ = sender.send(Event::Resize(w, h));
                }
                CrosstermEvent::FocusGained => {}
                CrosstermEvent::FocusLost => {}
                CrosstermEvent::Paste(_) => {}
            },
        }
    }
}
