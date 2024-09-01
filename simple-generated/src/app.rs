use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    widgets::{Block, BorderType, Paragraph},
    DefaultTerminal, Frame,
};

use crate::event::{Event, EventSource};

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        const TICKS_PER_SECOND: f64 = 10.0;
        let tick_interval = Duration::from_secs_f64(1.0 / TICKS_PER_SECOND);
        let events = EventSource::new(tick_interval);
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            match events.next()? {
                Event::Tick => self.on_tick(),
                Event::Key(key_event) => self.on_key_event(key_event)?,
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                Event::Error(err) => {
                    return Err(err.into());
                }
            }
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn on_tick(&self) {}

    /// Handles the key events and updates the state of [`App`].
    pub fn on_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Right) => self.increment_counter(),
            (_, KeyCode::Left) => self.decrement_counter(),
            // Add other key handlers here.
            _ => {}
        }
        Ok(())
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    /// Renders the user interface widgets.
    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
        // - https://github.com/ratatui/ratatui/tree/master/examples

        let text = format!(
            "This is a ratatui simple template.\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press left and right to increment and decrement the counter respectively.\n\
            Counter: {}",
            self.counter
        );
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Ratatui Simple Template")
            .title_alignment(Alignment::Center);
        frame.render_widget(
            Paragraph::new(text)
                .block(block)
                .style(Style::new().cyan().on_black())
                .centered(),
            frame.area(),
        )
    }
}
