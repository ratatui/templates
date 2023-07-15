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
pub enum Message {
  Render,
  Stop,
  Suspend,
}

pub struct App {
  pub tick_rate: (u64, u64),
  pub home: Arc<Mutex<Home>>,
}

impl App {
  pub fn new(tick_rate: (u64, u64)) -> Result<Self> {
    let home = Arc::new(Mutex::new(Home::new()));
    Ok(Self { tick_rate, home })
  }

  pub fn spawn_tui_task(&mut self) -> (JoinHandle<()>, mpsc::UnboundedSender<Message>) {
    let home = self.home.clone();

    let (tui_tx, mut tui_rx) = mpsc::unbounded_channel::<Message>();

    let tui_task = tokio::spawn(async move {
      let mut tui = TerminalHandler::new().context(anyhow!("Unable to create TUI")).unwrap();
      tui.enter().unwrap();
      loop {
        match tui_rx.recv().await {
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

    (tui_task, tui_tx)
  }

  pub fn spawn_event_task(&mut self, tx: mpsc::UnboundedSender<Action>) -> (JoinHandle<()>, CancellationToken) {
    let home = self.home.clone();
    let (app_tick_rate, render_tick_rate) = self.tick_rate;
    let event_handler_cancellation_token = CancellationToken::new();
    let cancellation_token = event_handler_cancellation_token.clone();
    let event_task = tokio::spawn(async move {
      let mut events = EventHandler::new(app_tick_rate, render_tick_rate);
      loop {
        tokio::select! {
          _ = cancellation_token.cancelled() => {
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
    (event_task, event_handler_cancellation_token)
  }

  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    self.home.lock().await.action_tx = Some(action_tx.clone());

    self.home.lock().await.init()?;

    let (mut tui_task, mut tui_tx) = self.spawn_tui_task();
    let (mut event_task, mut event_handler_cancellation_token) = self.spawn_event_task(action_tx.clone());

    loop {
      let mut maybe_action = action_rx.recv().await;
      while maybe_action.is_some() {
        let action = maybe_action.unwrap();
        if action == Action::RenderTick {
          tui_tx.send(Message::Render).unwrap_or_default();
        } else if action != Action::Tick {
          trace_dbg!(action);
        }
        if let Some(_action) = self.home.lock().await.dispatch(action) {
          action_tx.send(_action)?
        };
        maybe_action = action_rx.try_recv().ok();
      }

      if self.home.lock().await.should_suspend {
        tui_tx.send(Message::Suspend).unwrap_or_default();
        event_handler_cancellation_token.cancel();
        tui_task.await?;
        event_task.await?;
        (tui_task, tui_tx) = self.spawn_tui_task();
        (event_task, event_handler_cancellation_token) = self.spawn_event_task(action_tx.clone());
        action_tx.send(Action::Resume)?;
      } else if self.home.lock().await.should_quit {
        tui_tx.send(Message::Stop).unwrap_or_default();
        event_handler_cancellation_token.cancel();
        tui_task.await?;
        event_task.await?;
        break;
      }
    }
    Ok(())
  }
}
