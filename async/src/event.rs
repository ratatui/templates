use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::{
  sync::{mpsc, oneshot},
  task::JoinHandle,
};

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
  _tx: mpsc::UnboundedSender<Event>,
  rx: mpsc::UnboundedReceiver<Event>,
  stop: Option<oneshot::Sender<()>>,
  task: Option<JoinHandle<()>>,
}

impl EventHandler {
  pub fn new(tick_rate: u64) -> Self {
    let tick_rate = std::time::Duration::from_millis(tick_rate);

    let (tx, rx) = mpsc::unbounded_channel();
    let _tx = tx.clone();

    let (stop_tx, mut stop_rx) = oneshot::channel::<()>();

    let task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);
      loop {
        let delay = interval.tick();
        let crossterm_event = reader.next().fuse();
        if stop_rx.try_recv().ok().is_some() {
          break;
        }
        tokio::select! {
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
          _ = delay => {
              tx.send(Event::Tick).unwrap();
          },
        }
      }
    });

    Self { _tx, rx, stop: Some(stop_tx), task: Some(task) }
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.rx.recv().await
  }

  pub async fn stop(&mut self) -> Result<()> {
    if let Some(stop_tx) = self.stop.take() {
      stop_tx.send(()).unwrap()
    }
    if let Some(task) = self.task.take() {
      task.await.unwrap()
    }
    Ok(())
  }
}
