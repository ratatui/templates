use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    layout::Alignment,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph},
    Terminal,
};

use ratzilla::{event::KeyCode, DomBackend, WebRenderer};

fn main() -> io::Result<()> {
    let counter = Rc::new(RefCell::new(0u8));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.on_key_event({
        let counter_cloned = counter.clone();
        move |key_event| {
            let mut counter = counter_cloned.borrow_mut();
            match key_event.code {
                KeyCode::Left => {
                    *counter = counter.saturating_sub(1);
                }
                KeyCode::Right => {
                    *counter = counter.saturating_add(1);
                }
                _ => {}
            }
        }
    });

    terminal.draw_web(move |f| {
        let counter = counter.borrow();
        let block = Block::bordered()
            .title("web-generated")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        let text = format!(
            "This is a Ratatui/Ratzilla template.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {counter}",
        );
        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();
        f.render_widget(paragraph, f.area());
    });

    Ok(())
}
