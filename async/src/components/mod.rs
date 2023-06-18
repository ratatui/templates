use std::io::Stderr;

use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{backend::CrosstermBackend, layout::Rect, Frame as TuiFrame};

pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stderr>>;

use crate::{
  action::{Action, ActionHandler},
  event::Event,
};

pub mod home;
pub mod logger;

pub trait Component {
  fn init(&mut self) -> Result<()> {
    Ok(())
  }
  async fn handle_events(&self, event: Option<Event>, handler: &ActionHandler) -> Result<()> {
    match event {
      Some(Event::Quit) => handler.send(Action::Quit).await,
      Some(Event::Tick) => handler.send(Action::Tick).await,
      Some(Event::Key(key_event)) => self.handle_key_events(key_event, handler).await,
      Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event, handler).await,
      Some(Event::Resize(x, y)) => handler.send(Action::Resize(x, y)).await,
      Some(_) => handler.send(Action::Noop).await,
      None => handler.send(Action::Noop).await,
    }
  }
  async fn handle_key_events(&self, key: KeyEvent, handler: &ActionHandler) -> Result<()> {
    let action = self.on_key_event(key);
    handler.send(action).await
  }
  fn on_key_event(&self, key: KeyEvent) -> Action {
    Action::Noop
  }
  async fn handle_mouse_events(&self, mouse: MouseEvent, handler: &ActionHandler) -> Result<()> {
    let action = self.on_mouse_event(mouse);
    handler.send(action).await
  }
  fn on_mouse_event(&self, mouse: MouseEvent) -> Action {
    Action::Noop
  }
  async fn dispatch(&mut self, action: Action) -> Option<Action> {
    None
  }
  fn render(&mut self, f: &mut Frame<'_>, rect: Rect);
}
