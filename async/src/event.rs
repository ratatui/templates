use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tracing::{error, info, trace};

#[derive(Clone, Copy, Debug)]
pub enum Event {
  Quit,
  Error,
  Closed,
  Tick,
  Key(KeyEvent),
  Mouse(MouseEvent),
  Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
  tx: mpsc::UnboundedSender<Event>,
  rx: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
  pub fn new(tick_rate: u64) -> Self {
    let tick_rate = std::time::Duration::from_millis(tick_rate);

    let (tx, rx) = mpsc::unbounded_channel();
    let _tx = tx.clone();

    tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);
      loop {
        let delay = interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                      _tx.send(Event::Key(key)).unwrap();
                    }
                  },
                  CrosstermEvent::Resize(x, y) => {
                    _tx.send(Event::Resize(x, y)).unwrap();
                  },
                  _ => {},
                }
              }
              Some(Err(_)) => {
                _tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = delay => {
              _tx.send(Event::Tick).unwrap();
          },
        }
      }
    });

    Self { tx, rx }
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.rx.recv().await
  }
}
