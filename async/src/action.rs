use anyhow::Result;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
  Quit,
  Tick,
  Resize(u16, u16),
  ToggleShowLogger,
  IncrementCounter,
  DecrementCounter,
  EnterNormal,
  EnterInsert,
  EnterProcessing,
  Update,
  Noop,
}

#[derive(Debug)]
pub struct ActionHandler {
  pub tx: mpsc::UnboundedSender<Action>,
  pub rx: mpsc::UnboundedReceiver<Action>,
}

impl ActionHandler {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::unbounded_channel();
    Self { tx, rx }
  }

  pub fn try_recv(&mut self) -> Option<Action> {
    self.rx.try_recv().ok()
  }

  pub async fn recv(&mut self) -> Action {
    let action = self.rx.recv().await;
    action.unwrap_or(Action::Quit)
  }

  pub async fn send(&self, action: Action) -> Result<()> {
    self.tx.send(action)?;
    Ok(())
  }
}
