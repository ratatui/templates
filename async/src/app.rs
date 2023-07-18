use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{mpsc, Mutex};

use crate::{
  action::Action,
  components::{home::Home, Component},
  event::EventHandlerTask,
  terminal::TerminalHandlerTask,
  trace_dbg,
};

pub struct App {
  pub tick_rate: (u64, u64),
  pub home: Arc<Mutex<Home>>,
  pub should_quit: bool,
  pub should_suspend: bool,
}

impl App {
  pub fn new(tick_rate: (u64, u64)) -> Result<Self> {
    let home = Arc::new(Mutex::new(Home::new()));
    Ok(Self { tick_rate, home, should_quit: false, should_suspend: false })
  }

  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    self.home.lock().await.action_tx = Some(action_tx.clone());

    self.home.lock().await.init()?;

    let mut tui = TerminalHandlerTask::new(self.home.clone());
    let mut event = EventHandlerTask::new(self.home.clone(), self.tick_rate, action_tx.clone());

    loop {
      if let Some(action) = action_rx.recv().await {
        if action != Action::Tick {
          trace_dbg!(action);
        }
        match action {
          Action::RenderTick => tui.render()?,
          Action::Quit => self.should_quit = true,
          Action::Suspend => self.should_suspend = true,
          Action::Resume => self.should_suspend = false,
          _ => {
            if let Some(_action) = self.home.lock().await.dispatch(action) {
              action_tx.send(_action)?
            };
          },
        }
      }

      if self.should_suspend {
        tui.suspend()?;
        event.stop();
        tui.task.await?;
        event.task.await?;
        tui = TerminalHandlerTask::new(self.home.clone());
        event = EventHandlerTask::new(self.home.clone(), self.tick_rate, action_tx.clone());
        action_tx.send(Action::Resume)?;
      } else if self.should_quit {
        tui.stop()?;
        event.stop();
        tui.task.await?;
        event.task.await?;
        break;
      }
    }
    Ok(())
  }
}
