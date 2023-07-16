use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use tokio::{
  sync::{mpsc, Mutex},
  task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::{
  action::Action,
  components::{home::Home, Component},
  event::EventHandler,
  terminal::TerminalHandler,
  trace_dbg,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
  Render,
  Stop,
  Suspend,
}

struct TuiTask {
  task: JoinHandle<()>,
  tx: mpsc::UnboundedSender<Message>,
}

impl TuiTask {
  fn new(home: Arc<Mutex<Home>>) -> Self {
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let task = tokio::spawn(async move {
      let mut tui = TerminalHandler::new().context(anyhow!("Unable to create TUI")).unwrap();
      tui.enter().unwrap();
      loop {
        match rx.recv().await {
          Some(Message::Stop) => {
            tui.exit().unwrap_or_default();
            break;
          },
          Some(Message::Suspend) => {
            tui.suspend().unwrap_or_default();
            break;
          },
          Some(Message::Render) => {
            let mut h = home.lock().await;
            tui
              .terminal
              .draw(|f| {
                h.render(f, f.size());
              })
              .unwrap();
          },
          None => {},
        }
      }
    });
    Self { task, tx }
  }

  fn suspend(&self) -> Result<()> {
    self.tx.send(Message::Suspend)?;
    Ok(())
  }

  fn stop(&self) -> Result<()> {
    self.tx.send(Message::Stop)?;
    Ok(())
  }

  fn render(&self) -> Result<()> {
    self.tx.send(Message::Render)?;
    Ok(())
  }
}

struct EventTask {
  task: JoinHandle<()>,
  cancellation_token: CancellationToken,
}

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

  fn spawn_event_task(&mut self, tx: mpsc::UnboundedSender<Action>) -> EventTask {
    let home = self.home.clone();
    let (app_tick_rate, render_tick_rate) = self.tick_rate;
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
    EventTask { task, cancellation_token }
  }

  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    self.home.lock().await.action_tx = Some(action_tx.clone());

    self.home.lock().await.init()?;

    let mut tui = TuiTask::new(self.home.clone());
    let mut event = self.spawn_event_task(action_tx.clone());

    loop {
      let mut maybe_action = action_rx.recv().await;
      while maybe_action.is_some() {
        let action = maybe_action.take().unwrap();
        if action == Action::RenderTick {
          tui.render()?;
        } else if action != Action::Tick {
          trace_dbg!(action);
        }
        match action {
          Action::Quit => self.should_quit = true,
          Action::Suspend => self.should_suspend = true,
          Action::Resume => self.should_suspend = false,
          _ => {
            if let Some(_action) = self.home.lock().await.dispatch(action) {
              action_tx.send(_action)?
            };
            maybe_action = action_rx.try_recv().ok();
          },
        }
      }

      if self.should_suspend {
        tui.suspend()?;
        event.cancellation_token.cancel();
        tui.task.await?;
        event.task.await?;
        tui = TuiTask::new(self.home.clone());
        event = self.spawn_event_task(action_tx.clone());
        action_tx.send(Action::Resume)?;
      } else if self.should_quit {
        tui.stop()?;
        event.cancellation_token.cancel();
        tui.task.await?;
        event.task.await?;
        break;
      }
    }
    Ok(())
  }
}
