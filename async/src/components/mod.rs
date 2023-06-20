use std::io::Stderr;

use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{backend::CrosstermBackend, layout::Rect, Frame as TuiFrame};

pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stderr>>;

use crate::{app::Action, terminal::Event};

pub mod home;
pub mod logger;

pub trait Component {
  fn init(&mut self) -> Result<()> {
    Ok(())
  }
  fn handle_events(&mut self, event: Option<Event>) -> Action {
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
  fn handle_key_events(&mut self, key: KeyEvent) -> Action {
    self.on_key_event(key)
  }
  #[allow(unused_variables)]
  fn on_key_event(&mut self, key: KeyEvent) -> Action {
    Action::Noop
  }
  fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
    self.on_mouse_event(mouse)
  }
  #[allow(unused_variables)]
  fn on_mouse_event(&mut self, mouse: MouseEvent) -> Action {
    Action::Noop
  }
  #[allow(unused_variables)]
  fn dispatch(&mut self, action: Action) -> Option<Action> {
    None
  }
  fn render(&mut self, f: &mut Frame<'_>, rect: Rect);
}
