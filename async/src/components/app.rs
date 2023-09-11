use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use log::error;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use tracing::trace;
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{Component, Frame};
use crate::action::Action;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
  #[default]
  Normal,
  Insert,
  Processing,
}

#[derive(Default)]
pub struct App {
  pub show_help: bool,
  pub counter: usize,
  pub app_ticker: usize,
  pub render_ticker: usize,
  pub mode: Mode,
  pub input: Input,
  pub action_tx: Option<UnboundedSender<Action>>,
  pub keymap: HashMap<KeyEvent, Action>,
  pub text: Vec<String>,
}

impl App {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
    self.keymap = keymap;
    self
  }

  pub fn tick(&mut self) {
    trace!("Tick");
    self.app_ticker = self.app_ticker.saturating_add(1);
  }

  pub fn render_tick(&mut self) {
    trace!("Render Tick");
    self.render_ticker = self.render_ticker.saturating_add(1);
  }

  pub fn add(&mut self, s: String) {
    self.text.push(s)
  }

  pub fn schedule_increment(&mut self, i: usize) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      tokio::time::sleep(Duration::from_secs(1)).await;
      tx.send(Action::Increment(i)).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }

  pub fn schedule_decrement(&mut self, i: usize) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      tokio::time::sleep(Duration::from_secs(1)).await;
      tx.send(Action::Decrement(i)).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }

  pub fn increment(&mut self, i: usize) {
    self.counter = self.counter.saturating_add(i);
  }

  pub fn decrement(&mut self, i: usize) {
    self.counter = self.counter.saturating_sub(i);
  }
}

impl Component for App {
  fn init(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.action_tx = Some(tx);
    Ok(())
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
    let action = match self.mode {
      Mode::Normal | Mode::Processing => {
        if let Some(action) = self.keymap.get(&key) {
          action.clone()
        } else {
          Action::Tick
        }
      },
      Mode::Insert => match key.code {
        KeyCode::Esc => Action::EnterNormal,
        KeyCode::Enter => {
          if let Some(sender) = &self.action_tx {
            if let Err(e) = sender.send(Action::CompleteInput(self.input.value().to_string())) {
              error!("Failed to send action: {:?}", e);
            }
          }
          Action::EnterNormal
        },
        _ => {
          self.input.handle_event(&crossterm::event::Event::Key(key));
          Action::Update
        },
      },
    };
    Some(action)
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => self.tick(),
      Action::Render => self.render_tick(),
      Action::ToggleShowHelp => self.show_help = !self.show_help,
      Action::ScheduleIncrement => self.schedule_increment(1),
      Action::ScheduleDecrement => self.schedule_decrement(1),
      Action::Increment(i) => self.increment(i),
      Action::Decrement(i) => self.decrement(i),
      Action::CompleteInput(s) => self.add(s),
      Action::EnterNormal => {
        self.mode = Mode::Normal;
      },
      Action::EnterInsert => {
        self.mode = Mode::Insert;
      },
      Action::EnterProcessing => {
        self.mode = Mode::Processing;
      },
      Action::ExitProcessing => {
        // TODO: Make this go to previous mode instead
        self.mode = Mode::Normal;
      },
      _ => (),
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
    let rects = Layout::default().constraints([Constraint::Percentage(100), Constraint::Min(3)].as_ref()).split(rect);

    let mut text: Vec<Line> = self.text.clone().iter().map(|l| Line::from(l.clone())).collect();
    text.insert(0, "".into());
    text.insert(0, "Type into input and hit enter to display here".into());
    text.insert(0, "".into());
    text.insert(0, format!("Render Ticker: {}", self.render_ticker).into());
    text.insert(0, format!("App Ticker: {}", self.app_ticker).into());
    text.insert(0, format!("Counter: {}", self.counter).into());
    text.insert(0, "".into());
    text.insert(
      0,
      Line::from(vec![
        "Press ".into(),
        Span::styled("j", Style::default().fg(Color::Red)),
        " or ".into(),
        Span::styled("k", Style::default().fg(Color::Red)),
        " to ".into(),
        Span::styled("increment", Style::default().fg(Color::Yellow)),
        " or ".into(),
        Span::styled("decrement", Style::default().fg(Color::Yellow)),
        ".".into(),
      ]),
    );
    text.insert(0, "".into());

    f.render_widget(
      Paragraph::new(text)
        .block(
          Block::default()
            .title("ratatui async template")
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

    if self.show_help {
      let rect = rect.inner(&Margin { horizontal: 4, vertical: 2 });
      f.render_widget(Clear, rect);
      let block = Block::default()
        .title(Line::from(vec![Span::styled("Key Bindings", Style::default().add_modifier(Modifier::BOLD))]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
      f.render_widget(block, rect);
      let rows = vec![
        Row::new(vec!["j", "Increment"]),
        Row::new(vec!["k", "Decrement"]),
        Row::new(vec!["/", "Enter Input"]),
        Row::new(vec!["ESC", "Exit Input"]),
        Row::new(vec!["Enter", "Submit Input"]),
        Row::new(vec!["q", "Quit"]),
        Row::new(vec!["?", "Open Help"]),
      ];
      let table = Table::new(rows)
        .header(Row::new(vec!["Key", "Action"]).bottom_margin(1).style(Style::default().add_modifier(Modifier::BOLD)))
        .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)])
        .column_spacing(1);
      f.render_widget(table, rect.inner(&Margin { vertical: 4, horizontal: 2 }));
    };
  }
}
