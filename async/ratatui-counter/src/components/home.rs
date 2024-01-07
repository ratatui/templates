use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use log::error;
use ratatui::{
  prelude::*,
  widgets::{block::Title, *},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::trace;
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{Component, Frame};
use crate::{action::Action, config::key_event_to_string, tui::Event};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
  #[default]
  Normal,
  Insert,
  Processing,
  Help,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
  #[default]
  Normal,
  Hover,
  Clicked,
}

#[derive(Default)]
pub struct Home {
  pub show_help: bool,
  pub counter: usize,
  pub app_ticker: usize,
  pub render_ticker: usize,
  pub mode: Mode,
  pub input: Input,
  pub action_tx: Option<UnboundedSender<Action>>,
  pub keymap: HashMap<KeyEvent, Action>,
  pub text: Vec<String>,
  pub last_events: Vec<KeyEvent>,
  pub main_rect: Rect,
  pub input_rect: Rect,
  pub increment_rect: Rect,
  pub decrement_rect: Rect,
  pub increment_btn_state: ButtonState,
  pub decrement_btn_state: ButtonState,
}

impl Home {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
    self.keymap = keymap;
    self
  }

  pub fn tick(&mut self) {
    log::info!("Tick");
    self.app_ticker = self.app_ticker.saturating_add(1);
    self.last_events.drain(..);
  }

  pub fn render_tick(&mut self) {
    log::debug!("Render Tick");
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

  pub fn main_widget(&mut self) -> Paragraph<'_> {
    let mut text: Vec<Line> = self.text.clone().iter().map(|l| Line::from(l.clone())).collect();
    text.insert(0, "".into());
    text.insert(0, "Type into input and hit enter to display here".dim().into());
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
      .alignment(Alignment::Center)
  }

  fn input_widget(&mut self) -> Paragraph<'_> {
    let width = self.main_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = self.input.visual_scroll(width as usize);
    Paragraph::new(self.input.value())
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
      ])))
  }

  fn help_widget(&mut self) -> (Block<'_>, Table<'_>) {
    let block = Block::default()
      .title(Line::from(vec![Span::styled("Key Bindings", Style::default().add_modifier(Modifier::BOLD))]))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(Color::Yellow));
    let rows = vec![
      Row::new(vec!["j", "Increment"]),
      Row::new(vec!["k", "Decrement"]),
      Row::new(vec!["/", "Enter Input"]),
      Row::new(vec!["ESC", "Exit Input"]),
      Row::new(vec!["Enter", "Submit Input"]),
      Row::new(vec!["q", "Quit"]),
      Row::new(vec!["?", "Open Help"]),
    ];
    let table = Table::new(rows, &[Constraint::Percentage(10), Constraint::Percentage(90)])
      .header(Row::new(vec!["Key", "Action"]).bottom_margin(1).style(Style::default().add_modifier(Modifier::BOLD)))
      .column_spacing(1);
    (block, table)
  }

  fn title_widget(&mut self) -> Block<'_> {
    Block::default()
      .title(
        Title::from(format!("{:?}", &self.last_events.iter().map(|k| key_event_to_string(k)).collect::<Vec<_>>()))
          .alignment(Alignment::Right),
      )
      .title_style(Style::default().add_modifier(Modifier::BOLD))
  }

  pub fn increment_widget(&mut self) -> Paragraph<'_> {
    let color = if self.increment_btn_state == ButtonState::Hover {
      Color::Red
    } else if self.increment_btn_state == ButtonState::Clicked {
      Color::Blue
    } else {
      Color::Yellow
    };
    Paragraph::new("Increment").alignment(Alignment::Center).style(Style::new().bg(color))
  }

  pub fn decrement_widget(&mut self) -> Paragraph<'_> {
    let color = if self.decrement_btn_state == ButtonState::Hover {
      Color::Red
    } else if self.decrement_btn_state == ButtonState::Clicked {
      Color::Blue
    } else {
      Color::Yellow
    };
    Paragraph::new("Decrement").alignment(Alignment::Center).style(Style::new().bg(color))
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.action_tx = Some(tx);
    Ok(())
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    self.last_events.push(key.clone());
    let action = match self.mode {
      Mode::Normal | Mode::Processing | Mode::Help => return Ok(None),
      Mode::Insert => {
        match key.code {
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
        }
      },
    };
    Ok(Some(action))
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => self.tick(),
      Action::Render => self.render_tick(),
      Action::ToggleShowHelp if self.mode != Mode::Insert => {
        self.show_help = !self.show_help;
        if self.show_help {
          self.mode = Mode::Help;
        } else {
          self.mode = Mode::Normal;
        }
      },
      Action::ScheduleIncrement if self.mode == Mode::Normal => self.schedule_increment(1),
      Action::ScheduleDecrement if self.mode == Mode::Normal => self.schedule_decrement(1),
      Action::Increment(i) => self.increment(i),
      Action::Decrement(i) => self.decrement(i),
      Action::CompleteInput(s) => self.add(s),
      Action::EnterNormal => {
        self.mode = Mode::Normal;
        return Ok(Some(Action::EnterModeHomeNormal));
      },
      Action::EnterInsert => {
        self.mode = Mode::Insert;
        return Ok(Some(Action::EnterModeHomeInput));
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

  fn handle_mouse_events(&mut self, event: MouseEvent) -> Result<Option<Action>> {
    let MouseEvent { kind, column, row, modifiers } = event;
    // TODO: simulate better button clicks
    self.increment_btn_state = ButtonState::Normal;
    self.decrement_btn_state = ButtonState::Normal;
    if column >= self.increment_rect.left()
      && column <= self.increment_rect.right()
      && row >= self.increment_rect.top()
      && row <= self.increment_rect.bottom()
    {
      if kind == MouseEventKind::Moved {
        self.increment_btn_state = ButtonState::Hover;
      } else if kind == MouseEventKind::Down(MouseButton::Left) {
        self.increment_btn_state = ButtonState::Clicked;
        return Ok(Some(Action::ScheduleIncrement));
      } else if kind == MouseEventKind::Up(MouseButton::Left) {
        self.increment_btn_state = ButtonState::Hover;
      }
    };
    if column >= self.decrement_rect.left()
      && column <= self.decrement_rect.right()
      && row >= self.decrement_rect.top()
      && row <= self.decrement_rect.bottom()
    {
      if kind == MouseEventKind::Moved {
        self.decrement_btn_state = ButtonState::Hover;
      } else if kind == MouseEventKind::Down(MouseButton::Left) {
        self.decrement_btn_state = ButtonState::Clicked;
        return Ok(Some(Action::ScheduleDecrement));
      } else if kind == MouseEventKind::Up(MouseButton::Left) {
        self.decrement_btn_state = ButtonState::Hover;
      }
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
    let [main_rect, input_rect] =
      *Layout::default().constraints([Constraint::Percentage(100), Constraint::Min(3)].as_ref()).split(rect)
    else {
      panic!("Unable to split rects into a refutable pattern");
    };

    f.render_widget(self.main_widget(), main_rect);
    self.main_rect = main_rect;

    let buttons = Layout::default()
      .constraints([Constraint::Percentage(100), Constraint::Min(3), Constraint::Min(1)].as_ref())
      .split(main_rect)[1];
    let buttons = Layout::default()
      .constraints(
        [
          Constraint::Percentage(25),
          Constraint::Percentage(25),
          Constraint::Min(1),
          Constraint::Percentage(25),
          Constraint::Percentage(25),
        ]
        .as_ref(),
      )
      .direction(Direction::Horizontal)
      .split(buttons);

    f.render_widget(self.increment_widget(), buttons[1]);
    f.render_widget(self.decrement_widget(), buttons[3]);
    self.increment_rect = buttons[1];
    self.decrement_rect = buttons[3];

    f.render_widget(self.input_widget(), input_rect);
    self.input_rect = input_rect;

    if self.mode == Mode::Insert {
      f.set_cursor(
        (input_rect.x + 1 + self.input.cursor() as u16).min(input_rect.x + input_rect.width - 2),
        input_rect.y + 1,
      )
    }

    if self.show_help {
      let rect = rect.inner(&Margin { horizontal: 4, vertical: 2 });
      f.render_widget(Clear, rect);
      let (block, table) = self.help_widget();
      f.render_widget(block, rect);
      f.render_widget(table, rect.inner(&Margin { vertical: 4, horizontal: 2 }));
    };

    f.render_widget(self.title_widget(), Rect {
      x: rect.x + 1,
      y: rect.height.saturating_sub(1),
      width: rect.width.saturating_sub(2),
      height: 1,
    });

    Ok(())
  }
}
