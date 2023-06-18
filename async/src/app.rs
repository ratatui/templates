use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Context, Result};
use tokio::sync::Mutex;
use tracing::debug;

use crate::{
  action::{Action, ActionHandler},
  components::{home::Home, Component},
  event::EventHandler,
  tui::Tui,
};

pub struct App {
  pub events: EventHandler,
  pub actions: ActionHandler,
  pub home: Arc<Mutex<Home>>,
  pub tui: Arc<Mutex<Tui>>,
}

impl App {
  pub fn new(tick_rate: u64) -> Self {
    let tui = Arc::new(Mutex::new(Tui::new().context(anyhow!("Unable to create TUI")).unwrap()));
    let events = EventHandler::new(tick_rate);
    let actions = ActionHandler::new();
    let home = Arc::new(Mutex::new(Home::default()));
    Self { tui, events, actions, home }
  }

  pub async fn init(&mut self) -> Result<()> {
    self.home.lock().await.init()
  }

  pub async fn enter(&mut self) -> Result<()> {
    self.tui.lock().await.enter()?;
    Ok(())
  }

  pub async fn exit(&mut self) -> Result<()> {
    self.tui.lock().await.exit()?;
    Ok(())
  }

  pub async fn run(&mut self) -> Result<()> {
    let home = Arc::clone(&self.home);
    let tui = Arc::clone(&self.tui);
    tokio::spawn(async move {
      loop {
        let mut h = home.lock().await;
        let mut t = tui.lock().await;
        t.terminal
          .draw(|f| {
            h.render(f, f.size());
          })
          .unwrap();
        h.ticker = h.ticker.saturating_add(1);
        drop(h);
        tokio::time::sleep(Duration::from_millis(50)).await;
      }
    });

    loop {
      let event = self.events.next().await;
      self.home.lock().await.handle_events(event, &self.actions).await?;
      let mut action = Some(self.actions.recv().await);
      while action.is_some() {
        action = self.home.lock().await.dispatch(action.unwrap()).await;
      }
      if !(self.home.lock().await.is_running) {
        break;
      }
    }
    Ok(())
  }
}
