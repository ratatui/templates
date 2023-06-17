use anyhow::{anyhow, Context, Result};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
  Quit,
  Tick,
  Resize(u16, u16),
  ToggleShowLogger,
  IncrementCounter,
  DecrementCounter,
  Noop,
}

#[derive(Debug)]
pub struct ActionHandler {
  pub sender: mpsc::UnboundedSender<Action>,
  pub receiver: mpsc::UnboundedReceiver<Action>,
}

impl ActionHandler {
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::unbounded_channel();
    Self { sender, receiver }
  }

  pub async fn recv(&mut self) -> Action {
    let action = self.receiver.recv().await;
    debug!("Received action {:?}", action);
    action.unwrap_or(Action::Quit)
  }

  pub async fn send(&self, action: Action) -> Result<()> {
    debug!("Sending action {:?}", action);
    self.sender.send(action)?;
    Ok(())
  }
}
