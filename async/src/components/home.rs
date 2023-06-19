use std::{
  sync::{Arc, RwLock},
  time::Duration,
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Paragraph},
};
use tokio::sync::mpsc;
use tracing::trace;
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{logger::Logger, Component, Frame};
use crate::action::Action;

#[derive(Default, PartialEq, Eq)]
pub enum Mode {
  #[default]
  Normal,
  Insert,
  Processing,
}

pub struct Home {
  pub logger: Logger,
  pub should_quit: bool,
  pub show_logger: bool,
  pub counter: Arc<RwLock<usize>>,
  pub ticker: usize,
  pub mode: Mode,
  pub input: Input,
  pub actions: mpsc::UnboundedSender<Action>,
}

impl Home {
  pub fn new(actions: mpsc::UnboundedSender<Action>) -> Self {
    Self {
      logger: Default::default(),
      should_quit: Default::default(),
      show_logger: Default::default(),
      counter: Default::default(),
      ticker: Default::default(),
      mode: Default::default(),
      input: Default::default(),
      actions,
    }
  }

  pub fn tick(&mut self) {
    trace!("Tick");
    self.ticker = self.ticker.saturating_add(1);
  }

  pub fn increment(&mut self, i: usize) {
    let counter = self.counter.clone();
    let actions = self.actions.clone();
    actions.send(Action::EnterProcessing).unwrap();
    tokio::task::spawn(async move {
      tokio::time::sleep(Duration::from_secs(5)).await;
      let mut counter = counter.write().unwrap();
      *counter = counter.saturating_add(i);
      actions.send(Action::EnterNormal).unwrap();
    });
  }

  pub fn decrement(&mut self, i: usize) {
    let counter = self.counter.clone();
    let actions = self.actions.clone();
    actions.send(Action::EnterProcessing).unwrap();
    tokio::task::spawn(async move {
      tokio::time::sleep(Duration::from_secs(5)).await;
      let mut counter = counter.write().unwrap();
      *counter = counter.saturating_sub(i);
      actions.send(Action::EnterNormal).unwrap();
    });
  }

  pub fn counter(&self) -> usize {
    *(self.counter.read().unwrap())
  }

  pub fn is_running(&self) -> bool {
    self.should_quit
  }
}

impl Component for Home {
  fn on_key_event(&mut self, key: KeyEvent) -> Action {
    match self.mode {
      Mode::Normal | Mode::Processing => {
        match key.code {
          KeyCode::Char('q') => Action::Quit,
          KeyCode::Char('l') => Action::ToggleShowLogger,
          KeyCode::Char('j') => Action::IncrementCounter,
          KeyCode::Char('k') => Action::DecrementCounter,
          KeyCode::Char('i') => Action::EnterInsert,
          _ => Action::Tick,
        }
      },
      Mode::Insert => {
        match key.code {
          KeyCode::Esc => Action::EnterNormal,
          KeyCode::Enter => Action::EnterNormal,
          _ => {
            self.input.handle_event(&crossterm::event::Event::Key(key));
            Action::Update
          },
        }
      },
    }
  }

  fn dispatch(&mut self, action: Action) -> Option<Action> {
    match action {
      Action::Quit => self.should_quit = true,
      Action::Tick => self.tick(),
      Action::ToggleShowLogger => self.show_logger = !self.show_logger,
      Action::IncrementCounter => {
        self.increment(1);
        return Some(Action::Tick);
      },
      Action::DecrementCounter => {
        self.decrement(1);
        return Some(Action::Tick);
      },
      Action::EnterNormal => self.mode = Mode::Normal,
      Action::EnterInsert => self.mode = Mode::Insert,
      Action::EnterProcessing => self.mode = Mode::Processing,
      _ => (),
    }
    None
  }

  fn render(&mut self, f: &mut Frame<'_>, rect: Rect) {
    let rect = if self.show_logger {
      let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);
      self.logger.render(f, chunks[1]);
      chunks[0]
    } else {
      rect
    };

    let rects = Layout::default().constraints([Constraint::Percentage(100), Constraint::Min(3)].as_ref()).split(rect);

    f.render_widget(
      Paragraph::new(format!(
        "Press j or k to increment or decrement.\n\nCounter: {}\n\nTicker: {}",
        self.counter(),
        self.ticker
      ))
      .block(
        Block::default()
          .title("Template")
          .title_alignment(Alignment::Center)
          .borders(Borders::ALL)
          .border_style(match self.mode {
            Mode::Processing => Style::default().fg(Color::Yellow),
            _ => Style::default(),
          })
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Cyan))
      .alignment(Alignment::Center),
      rects[0],
    );
    let width = rects[1].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = self.input.visual_scroll(width as usize);
    let input = Paragraph::new(self.input.value())
      .style(match self.mode {
        Mode::Insert => Style::default().fg(Color::Yellow),
        _ => Style::default(),
      })
      .scroll((0, scroll as u16))
      .block(Block::default().borders(Borders::ALL).title(Line::from(vec![
        Span::raw("Enter Input Mode "),
        Span::styled("(Press ", Style::default().fg(Color::DarkGray)),
        Span::styled("/", Style::default().add_modifier(Modifier::BOLD).fg(Color::Gray)),
        Span::styled(" to start, ", Style::default().fg(Color::DarkGray)),
        Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD).fg(Color::Gray)),
        Span::styled(" to finish)", Style::default().fg(Color::DarkGray)),
      ])));
    f.render_widget(input, rects[1]);
    if self.mode == Mode::Insert {
      f.set_cursor((rects[1].x + 1 + self.input.cursor() as u16).min(rects[1].x + rects[1].width - 2), rects[1].y + 1)
    }
  }
}
