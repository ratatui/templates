use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use tokio::sync::Mutex;

use crate::{
  action::{Action, ActionHandler},
  components::{home::Home, Component},
  event::EventHandler,
  trace_dbg,
  tui::Tui,
};

pub struct App {
  pub events: EventHandler,
  pub actions: ActionHandler,
  pub home: Arc<Mutex<Home>>,
  pub tui: Arc<Mutex<Tui>>,
}

impl App {
  pub fn new(tick_rate: u64) -> Result<Self> {
    let events = EventHandler::new(tick_rate);
    let actions = ActionHandler::new();
    let tui = Arc::new(Mutex::new(Tui::new().context(anyhow!("Unable to create TUI")).unwrap()));
    let home = Arc::new(Mutex::new(Home::new(actions.tx.clone())));
    Ok(Self { tui, events, actions, home })
  }

  pub async fn run(&mut self) -> Result<()> {
    let home = Arc::clone(&self.home);
    let tui = Arc::clone(&self.tui);
    home.lock().await.init()?;
    tokio::spawn(async move {
      loop {
        let mut h = home.lock().await;
        let mut t = tui.lock().await;
        t.terminal
          .draw(|f| {
            h.render(f, f.size());
          })
          .unwrap();
      }
    });

    loop {
      // get the next event
      let event = self.events.next().await;

      // map event to an action
      let action = self.home.lock().await.handle_events(event);

      // add action to action handler channel queue
      self.actions.send(action).await?;

      // clear all actions from action handler channel queue
      let mut maybe_action = self.actions.try_recv();
      while maybe_action.is_some() {
        let action = maybe_action.unwrap();
        if action != Action::Tick {
          trace_dbg!(action);
        }
        if let Some(action) = self.home.lock().await.dispatch(action) {
          self.actions.send(action).await?
        };
        maybe_action = self.actions.try_recv();
      }

      // quit state
      if self.home.lock().await.should_quit {
        break;
      }
    }
    Ok(())
  }
}
