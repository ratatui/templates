use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.counter
        ))
        .block(
            Block::default()
                .title("Template")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        frame.size(),
    )
}
