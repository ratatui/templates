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
