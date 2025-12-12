use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use crossterm_keybind::KeyBindTrait;
use app_event::AppKeyEvent;
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

mod app_event;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // NOTE You can pass Some(path) and let user customize keybindings in an easy way
    AppKeyEvent::init_and_load(None)?;

    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = format!(r#"Hello, Ratatui!

Created using https://github.com/ratatui/templates
Press {} to stop running.

--- you can easy to change the keybind by loading following keybing config
{}
"#,
            AppKeyEvent::Quit.key_bindings_display(),
            AppKeyEvent::toml_example(),
        );
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title)),
            frame.area(),
        )
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        if AppKeyEvent::Quit.match_any(&key) {
            self.quit()
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
