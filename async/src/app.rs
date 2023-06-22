use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use tokio::{
  sync::{mpsc, oneshot, Mutex},
  task::JoinHandle,
};
use tracing::debug;

use crate::{
  components::{home::Home, Component},
  terminal::{EventHandler, TerminalHandler},
  trace_dbg,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
  Quit,
  Resume,
  Suspend,
  Tick,
  Resize(u16, u16),
  ToggleShowLogger,
  ScheduleIncrementCounter,
  ScheduleDecrementCounter,
  AddToCounter(usize),
  SubtractFromCounter(usize),
  EnterNormal,
  EnterInsert,
  EnterProcessing,
  ExitProcessing,
  Update,
  Noop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiMsg {
  Render,
  Stop,
}

pub struct App {
  pub tick_rate: u64,
  pub home: Arc<Mutex<Home>>,
}

impl App {
  pub fn new(tick_rate: u64) -> Result<Self> {
    let home = Arc::new(Mutex::new(Home::new()));
    Ok(Self { tick_rate, home })
  }

  pub fn spawn_tui_task(&mut self) -> (JoinHandle<()>, mpsc::UnboundedSender<TuiMsg>) {
    let home = self.home.clone();

    let (tui_tx, mut tui_rx) = mpsc::unbounded_channel::<TuiMsg>();

    let tui_task = tokio::spawn(async move {
      let mut tui = TerminalHandler::new().context(anyhow!("Unable to create TUI")).unwrap();
      tui.enter().unwrap();
      loop {
        match tui_rx.recv().await {
          Some(TuiMsg::Stop) => break,
          Some(TuiMsg::Render) => {
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
      tui.exit().unwrap();
    });

    (tui_task, tui_tx)
  }

  pub fn spawn_event_task(&mut self, tx: mpsc::UnboundedSender<Action>) -> (JoinHandle<()>, oneshot::Sender<()>) {
    let home = self.home.clone();
    let tick_rate = self.tick_rate;
    let (stop_event_tx, mut stop_event_rx) = oneshot::channel::<()>();
    let event_task = tokio::spawn(async move {
      let mut events = EventHandler::new(tick_rate);
      loop {
        let event = events.next().await;
        let action = home.lock().await.handle_events(event);
        tx.send(action).unwrap();

        if stop_event_rx.try_recv().ok().is_some() {
          events.stop().await.unwrap();
          break;
        }
      }
    });
    (event_task, stop_event_tx)
  }

  pub async fn run(&mut self) -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();

    self.home.lock().await.tx = Some(tx.clone());

    self.home.lock().await.init()?;

    let (mut tui_task, mut tui_tx) = self.spawn_tui_task();
    let (mut event_task, mut stop_event_tx) = self.spawn_event_task(tx.clone());

    loop {
      // clear all actions from action handler channel queue
      let mut maybe_action = rx.recv().await;
      while maybe_action.is_some() {
        let action = maybe_action.unwrap();
        if action != Action::Tick {
          trace_dbg!(action);
        } else {
          tui_tx.send(TuiMsg::Render).unwrap_or(());
        }
        if let Some(action) = self.home.lock().await.dispatch(action) {
          tx.send(action)?
        };
        maybe_action = rx.try_recv().ok();
      }

      if self.home.lock().await.should_suspend {
        tui_tx.send(TuiMsg::Stop).unwrap_or(());
        stop_event_tx.send(()).unwrap_or(());
        tui_task.await?;
        event_task.await?;
        let tui = TerminalHandler::new().context(anyhow!("Unable to create TUI")).unwrap();
        tui.suspend()?; // Blocks here till process resumes on Linux and Mac.
                        // TODO: figure out appropriate behaviour on Windows.
        debug!("resuming");
        (tui_task, tui_tx) = self.spawn_tui_task();
        (event_task, stop_event_tx) = self.spawn_event_task(tx.clone());
        tx.send(Action::Resume)?;
      } else if self.home.lock().await.should_quit {
        tui_tx.send(TuiMsg::Stop).unwrap_or(());
        stop_event_tx.send(()).unwrap_or(());
        tui_task.await?;
        event_task.await?;
        break;
      }
    }
    Ok(())
  }
}
