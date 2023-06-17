use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
// use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
// use signal_hook_tokio::{Signals, SignalsInfo};
use tokio::sync::mpsc;
use tracing::{error, info, trace};

#[derive(Clone, Copy, Debug)]
pub enum Event {
  Quit,
  Closed,
  Tick,
  Key(KeyEvent),
  Mouse(MouseEvent),
  Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
  sender: mpsc::UnboundedSender<Event>,
  receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
  pub fn new(tick_rate: u64) -> Self {
    let tick_rate = std::time::Duration::from_millis(tick_rate);

    let (sender, receiver) = mpsc::unbounded_channel();
    let _sender = sender.clone();

    tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      // let mut signals =
      //   Signals::new([SIGHUP, SIGTERM, SIGINT, SIGQUIT]).context(anyhow!("Unable to create Signals handler.")).unwrap();
      loop {
        let delay = tokio::time::sleep(tick_rate).fuse();
        let crossterm_event = reader.next().fuse();
        // let signal = signals.next().fuse();

        tokio::select! {
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                      _sender.send(Event::Key(key)).unwrap();
                    }
                  },
                  CrosstermEvent::Resize(x, y) => {
                    _sender.send(Event::Resize(x, y)).unwrap();
                  },
                  _ => {},
                }
              }
              Some(Err(_)) => {}
              None => {},
            }
          },
          // maybe_signal = signal => {
          //   match maybe_signal {
          //     Some(s) => {
          //       match s {
          //           SIGTERM | SIGINT | SIGQUIT => {
          //               info!("Received signal {}, shutting down", s);
          //               _sender.send(Event::Quit).unwrap()
          //           }
          //           _ => {
          //               error!("Received unexpected signal {}", s);
          //           }
          //       }
          //     }
          //     None => {},
          //   }
          // },
          _ = delay => {
              _sender.send(Event::Tick).unwrap();
          },
        }
      }
    });

    Self { sender, receiver }
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.receiver.recv().await
  }
}
