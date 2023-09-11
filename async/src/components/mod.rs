use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, tui::Event, tui::Frame};

pub mod app;

//// ANCHOR: component
pub trait Component {
  #[allow(unused_variables)]
  fn init(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    Ok(())
  }
  fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
    let action = match event {
      Some(Event::Quit) => Action::Quit,
      Some(Event::Tick) => Action::Tick,
      Some(Event::Render) => Action::Render,
      Some(Event::Key(key_event)) => return self.handle_key_events(key_event),
      Some(Event::Mouse(mouse_event)) => return self.handle_mouse_events(mouse_event),
      Some(Event::Resize(x, y)) => Action::Resize(x, y),
      Some(_) => return None,
      None => return None,
    };
    Some(action)
  }
  #[allow(unused_variables)]
  fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
    None
  }
  #[allow(unused_variables)]
  fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Action> {
    None
  }
  #[allow(unused_variables)]
  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    Ok(None)
  }
  fn draw(&mut self, f: &mut Frame<'_>, rect: Rect);
}
//// ANCHOR_END: component
