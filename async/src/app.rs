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
  pub home: Home,
  pub tui: Tui,
}

impl App {
  pub fn new(tick_rate: u64) -> Self {
    let tui = Tui::new().context(anyhow!("Unable to create TUI")).unwrap();
    let events = EventHandler::new(tick_rate);
    let actions = ActionHandler::new();
    let home = Home::default();
    Self { tui, events, actions, home }
  }

  pub async fn init(&mut self) -> Result<()> {
    self.home.init()
  }

  pub async fn enter(&mut self) -> Result<()> {
    self.tui.enter()?;
    Ok(())
  }

  pub async fn exit(&mut self) -> Result<()> {
    self.tui.exit()?;
    Ok(())
  }

  pub async fn run(&mut self) -> Result<()> {
    loop {
      self
        .tui
        .terminal
        .draw(|f| {
          self.home.render(f, f.size());
        })
        .unwrap();
      self.home.ticker = self.home.ticker.saturating_add(1);
      let event = self.events.next().await;
      let mut action = Some(self.home.handle_events(event).await);
      while action.is_some() {
        action = self.home.dispatch(action.unwrap()).await;
      }
      if !(self.home.is_running) {
        break;
      }
    }
    Ok(())
  }
}
