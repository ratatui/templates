use std::io::Stderr;

use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{backend::CrosstermBackend, layout::Rect, Frame as TuiFrame};

pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stderr>>;

use crate::{action::Action, event::Event};

pub mod home;
pub mod logger;

pub trait Component {
  fn init(&mut self) -> Result<()> {
    Ok(())
  }
  fn handle_events(&self, event: Option<Event>) -> Action {
    match event {
      Some(Event::Quit) => Action::Quit,
      Some(Event::Tick) => Action::Tick,
      Some(Event::Key(key_event)) => self.handle_key_events(key_event),
      Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event),
      Some(Event::Resize(x, y)) => Action::Resize(x, y),
      Some(_) => Action::Noop,
      None => Action::Noop,
    }
  }
  fn handle_key_events(&self, key: KeyEvent) -> Action {
    self.on_key_event(key)
  }
  fn on_key_event(&self, key: KeyEvent) -> Action {
    Action::Noop
  }
  fn handle_mouse_events(&self, mouse: MouseEvent) -> Action {
    self.on_mouse_event(mouse)
  }
  fn on_mouse_event(&self, mouse: MouseEvent) -> Action {
    Action::Noop
  }
  fn dispatch(&mut self, action: Action) -> Option<Action> {
    None
  }
  fn render(&mut self, f: &mut Frame<'_>, rect: Rect);
}
