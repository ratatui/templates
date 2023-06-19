use anyhow::{anyhow, Context, Result};

use crate::{
  action::ActionHandler,
  components::{home::Home, Component},
  event::EventHandler,
  trace_dbg,
  tui::Tui,
};

pub struct App {
  pub events: EventHandler,
  pub home: Home,
  pub actions: ActionHandler,
  pub tui: Tui,
}

impl App {
  pub fn new(tick_rate: u64) -> Result<Self> {
    let tui = Tui::new().context(anyhow!("Unable to create TUI")).unwrap();
    let events = EventHandler::new(tick_rate);
    let actions = ActionHandler::new();
    let mut home = Home::new(actions.tx.clone());
    home.init()?;
    Ok(Self { tui, events, actions, home })
  }

  pub async fn run(&mut self) -> Result<()> {
    loop {
      // draw to terminal
      self
        .tui
        .terminal
        .draw(|f| {
          self.home.render(f, f.size());
        })
        .unwrap();

      // get the next event
      let event = self.events.next().await;

      // map event to an action
      let action = self.home.handle_events(event);

      // add action to action handler channel queue
      self.actions.send(action).await?;

      // clear all actions from action handler channel queue
      let mut maybe_action = self.actions.try_recv();
      while maybe_action.is_some() {
        let action = maybe_action.unwrap();
        trace_dbg!(action);
        if let Some(action) = self.home.dispatch(action) {
          self.actions.send(action).await?
        };
        maybe_action = self.actions.try_recv();
      }

      // quit state
      if self.home.should_quit {
        break;
      }
    }
    Ok(())
  }
}
