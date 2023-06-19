use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use tokio::{
  sync::{mpsc, oneshot, Mutex},
  task::JoinHandle,
};

use crate::{
  action::Action,
  components::{home::Home, Component},
  event::EventHandler,
  trace_dbg,
  tui::Tui,
};

pub struct App {
  pub tick_rate: u64,
  pub stop_tui_tx: Option<oneshot::Sender<()>>,
  pub tui_task: Option<JoinHandle<()>>,
  pub tx: mpsc::UnboundedSender<Action>,
  pub rx: mpsc::UnboundedReceiver<Action>,
  pub home: Arc<Mutex<Home>>,
}

impl App {
  pub fn new(tick_rate: u64) -> Result<Self> {
    let (tx, rx) = mpsc::unbounded_channel();
    let home = Arc::new(Mutex::new(Home::new(tx.clone())));
    Ok(Self { tick_rate, tx, rx, home, stop_tui_tx: None, tui_task: None })
  }

  pub fn spawn_tui_task(&mut self) {
    let home = self.home.clone();

    let (stop_tui_tx, mut rx) = oneshot::channel::<()>();

    let tui_task = tokio::spawn(async move {
      let mut tui = Tui::new().context(anyhow!("Unable to create TUI")).unwrap();
      tui.enter().unwrap();
      loop {
        let mut h = home.lock().await;
        tui
          .terminal
          .draw(|f| {
            h.render(f, f.size());
          })
          .unwrap();
        if rx.try_recv().ok().is_some() {
          break;
        }
      }
      tui.exit().unwrap();
    });

    self.tui_task = Some(tui_task);
    self.stop_tui_tx = Some(stop_tui_tx);
  }

  pub fn spawn_event_task(&mut self) {
    let home = self.home.clone();
    let tx = self.tx.clone();
    let tick_rate = self.tick_rate;
    tokio::spawn(async move {
      let mut events = EventHandler::new(tick_rate);
      loop {
        // get the next event
        let event = events.next().await;

        // map event to an action
        let action = home.lock().await.handle_events(event);

        // add action to action handler channel queue
        tx.send(action).unwrap();
      }
    });
  }

  pub async fn run(&mut self) -> Result<()> {
    self.home.lock().await.init()?;

    self.spawn_tui_task();
    self.spawn_event_task();

    loop {
      // clear all actions from action handler channel queue
      let mut maybe_action = self.rx.try_recv().ok();
      while maybe_action.is_some() {
        let action = maybe_action.unwrap();
        if action != Action::Tick {
          trace_dbg!(action);
        }
        if let Some(action) = self.home.lock().await.dispatch(action) {
          self.tx.send(action)?
        };
        maybe_action = self.rx.try_recv().ok();
      }

      // quit state
      if self.home.lock().await.should_quit {
        if let Some(tx) = self.stop_tui_tx.take() {
          tx.send(()).unwrap_or_else(|_| ());
        }
        if let Some(h) = self.tui_task.take() {
          h.await?;
        }
        break;
      }
    }
    Ok(())
  }
}
