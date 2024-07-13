use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::{
  cursor,
  event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
    Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent, 
  },
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend as Backend;
use serde::{Deserialize, Serialize};
use tokio::{
  sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
  task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

pub type IO = std::io::{{crossterm_io | title_case}};
pub fn io() -> IO {
  std::io::{{crossterm_io}}()
}
pub type Frame<'a> = ratatui::Frame<'a>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
  Init,
  Quit,
  Error,
  Closed,
  Tick,
  Render,
  FocusGained,
  FocusLost,
  Paste(String),
  Key(KeyEvent),
  Mouse(MouseEvent),
  Resize(u16, u16),
}

pub struct Tui {
  pub terminal: ratatui::Terminal<Backend<IO>>,
  pub task: JoinHandle<()>,
  pub cancellation_token: CancellationToken,
  pub event_rx: UnboundedReceiver<Event>,
  pub event_tx: UnboundedSender<Event>,
  pub frame_rate: f64,
  pub tick_rate: f64,
  pub mouse: bool,
  pub paste: bool,
}

impl Tui {
  pub fn new() -> Result<Self> {
    let tick_rate = 4.0;
    let frame_rate = 60.0;
    let terminal = ratatui::Terminal::new(Backend::new(io()))?;
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let cancellation_token = CancellationToken::new();
    let task = tokio::spawn(async {});
    let mouse = false;
    let paste = false;
    Ok(Self { terminal, task, cancellation_token, event_rx, event_tx, frame_rate, tick_rate, mouse, paste })
  }

  pub fn tick_rate(mut self, tick_rate: f64) -> Self {
    self.tick_rate = tick_rate;
    self
  }

  pub fn frame_rate(mut self, frame_rate: f64) -> Self {
    self.frame_rate = frame_rate;
    self
  }

  pub fn mouse(mut self, mouse: bool) -> Self {
    self.mouse = mouse;
    self
  }

  pub fn paste(mut self, paste: bool) -> Self {
    self.paste = paste;
    self
  }

  pub fn start(&mut self) {
    self.cancel(); // Cancel any existing task
    self.cancellation_token = CancellationToken::new();
    let cancellation_token = self.cancellation_token.clone();
    let event_tx = self.event_tx.clone();
    let mut event_stream = EventStream::new();
    let mut tick_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / self.tick_rate));
    let mut render_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / self.frame_rate));
    self.task = tokio::spawn(async move {
      event_tx.send(Event::Init).expect("failed to send init event"); // likely a bug if this fails
      loop {
        let event = tokio::select! {
          _ = cancellation_token.cancelled() => {
            break;
          }
          crossterm_event = event_stream.next().fuse() => match crossterm_event {
            Some(Ok(event)) => match event {
              CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Event::Key(key),
              CrosstermEvent::Mouse(mouse) => Event::Mouse(mouse),
              CrosstermEvent::Resize(x, y) => Event::Resize(x, y),
              CrosstermEvent::FocusLost => Event::FocusLost,
              CrosstermEvent::FocusGained => Event::FocusGained,
              CrosstermEvent::Paste(s) => Event::Paste(s),
              _ => continue, // ignore other events
            }
            Some(Err(_)) => Event::Error,
            None => break, // the event stream has stopped and will not produce any more events
          },
          _ = tick_interval.tick() => Event::Tick,
          _ = render_interval.tick() => Event::Render,
        };
        if event_tx.send(event).is_err() {
          // the receiver has been dropped, so there's no point in continuing the loop
          break;
        }
      }
    });
  }

  pub fn stop(&self) -> Result<()> {
    self.cancel();
    let mut counter = 0;
    while !self.task.is_finished() {
      std::thread::sleep(Duration::from_millis(1));
      counter += 1;
      if counter > 50 {
        self.task.abort();
      }
      if counter > 100 {
        log::error!("Failed to abort task in 100 milliseconds for unknown reason");
        break;
      }
    }
    Ok(())
  }

  pub fn enter(&mut self) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io(), EnterAlternateScreen, cursor::Hide)?;
    if self.mouse {
      crossterm::execute!(io(), EnableMouseCapture)?;
    }
    if self.paste {
      crossterm::execute!(io(), EnableBracketedPaste)?;
    }
    self.start();
    Ok(())
  }

  pub fn exit(&mut self) -> Result<()> {
    self.stop()?;
    if crossterm::terminal::is_raw_mode_enabled()? {
      self.flush()?;
      if self.paste {
        crossterm::execute!(io(), DisableBracketedPaste)?;
      }
      if self.mouse {
        crossterm::execute!(io(), DisableMouseCapture)?;
      }
      crossterm::execute!(io(), LeaveAlternateScreen, cursor::Show)?;
      crossterm::terminal::disable_raw_mode()?;
    }
    Ok(())
  }

  pub fn cancel(&self) {
    self.cancellation_token.cancel();
  }

  pub fn suspend(&mut self) -> Result<()> {
    self.exit()?;
    #[cfg(not(windows))]
    signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
    Ok(())
  }

  pub fn resume(&mut self) -> Result<()> {
    self.enter()?;
    Ok(())
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.event_rx.recv().await
  }
}

impl Deref for Tui {
  type Target = ratatui::Terminal<Backend<IO>>;

  fn deref(&self) -> &Self::Target {
    &self.terminal
  }
}

impl DerefMut for Tui {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.terminal
  }
}

impl Drop for Tui {
  fn drop(&mut self) {
    self.exit().unwrap();
  }
}
