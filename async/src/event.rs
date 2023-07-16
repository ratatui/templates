use std::sync::Arc;

use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::{
  sync::{mpsc, Mutex},
  task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::{
  action::Action,
  components::{home::Home, Component},
};

#[derive(Clone, Copy, Debug)]
pub enum Event {
  Quit,
  Error,
  Closed,
  RenderTick,
  AppTick,
  Key(KeyEvent),
  Mouse(MouseEvent),
  Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
  _tx: mpsc::UnboundedSender<Event>,
  rx: mpsc::UnboundedReceiver<Event>,
  stop_cancellation_token: CancellationToken,
  task: Option<JoinHandle<()>>,
}

impl EventHandler {
  pub fn new(app_tick_rate: u64, render_tick_rate: u64) -> Self {
    let app_tick_rate = std::time::Duration::from_millis(app_tick_rate);
    let render_tick_rate = std::time::Duration::from_millis(render_tick_rate);

    let (tx, rx) = mpsc::unbounded_channel();
    let _tx = tx.clone();

    let stop_cancellation_token = CancellationToken::new();
    let _stop_cancellation_token = stop_cancellation_token.clone();

    let task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut app_interval = tokio::time::interval(app_tick_rate);
      let mut render_interval = tokio::time::interval(render_tick_rate);
      loop {
        let app_delay = app_interval.tick();
        let render_delay = render_interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          _ = _stop_cancellation_token.cancelled() => {
            break;
          }
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                      tx.send(Event::Key(key)).unwrap();
                    }
                  },
                  CrosstermEvent::Resize(x, y) => {
                    tx.send(Event::Resize(x, y)).unwrap();
                  },
                  _ => {},
                }
              }
              Some(Err(_)) => {
                tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = app_delay => {
              tx.send(Event::AppTick).unwrap();
          },
          _ = render_delay => {
              tx.send(Event::RenderTick).unwrap();
          },
        }
      }
    });

    Self { _tx, rx, stop_cancellation_token, task: Some(task) }
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.rx.recv().await
  }

  pub async fn stop(&mut self) -> Result<()> {
    self.stop_cancellation_token.cancel();
    if let Some(handle) = self.task.take() {
      handle.await.unwrap();
    }
    Ok(())
  }
}

pub struct EventHandlerTask {
  pub task: JoinHandle<()>,
  cancellation_token: CancellationToken,
}

impl EventHandlerTask {
  pub fn new(home: Arc<Mutex<Home>>, tick_rate: (u64, u64), tx: mpsc::UnboundedSender<Action>) -> EventHandlerTask {
    let (app_tick_rate, render_tick_rate) = tick_rate;
    let cancellation_token = CancellationToken::new();
    let _cancellation_token = cancellation_token.clone();
    let task = tokio::spawn(async move {
      let mut events = EventHandler::new(app_tick_rate, render_tick_rate);
      loop {
        tokio::select! {
          _ = _cancellation_token.cancelled() => {
            events.stop().await.unwrap();
            break;
          }
          event = events.next() => {
            let action = home.lock().await.handle_events(event);
            tx.send(action).unwrap();
          }
        }
      }
    });
    Self { task, cancellation_token }
  }

  pub fn stop(&mut self) {
    self.cancellation_token.cancel();
  }
}
