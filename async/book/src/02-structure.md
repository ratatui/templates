# `terminal.rs`

In this section of the tutorial, we are going to discuss the basic components of the `Tui` struct.

You'll find most people setup and teardown of a terminal application using `crossterm` like so:

```rust
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
  let mut stdout = io::stdout();
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture, HideCursor)?;
  Terminal::new(CrosstermBackend::new(stdout))
}

fn teardown_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
  let mut stdout = io::stdout();
  crossterm::terminal::disable_raw_mode()?;
  crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, ShowCursor)?;
  Ok(())
}

fn main() -> Result<()> {
  let mut terminal = setup_terminal()?;
  run_app(&mut terminal)?;
  teardown_terminal(&mut terminal)?;
  Ok(())
}
```

You can use `termion` or `termwiz` instead here, and you'll have to change the implementation of `setup_terminal` and `teardown_terminal`.

I personally like to use `crossterm` so that I can run the TUI on windows as well.

```admonish note
Terminals have two screen buffers for each window.
The default screen buffer is what you are dropped into when you start up a terminal.
The second screen buffer, called the alternate screen, is used for running interactive apps such as the `vim`, `less` etc.

Here's a 8 minute talk on Terminal User Interfaces I gave at JuliaCon2020: <https://www.youtube.com/watch?v=-TASx67pphw> that might be worth watching for more information about how terminal user interfaces work.
```

We can reorganize the setup and teardown functions into an `enter()` and `exit()` methods on a `Tui` struct.

```rust {filename="terminal.rs"}
use anyhow::Result;
use crossterm::{
  cursor,
  event::{DisableMouseCapture, EnableMouseCapture},
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend as Backend;

pub type Frame<'a> = ratatui::Frame<'a, Backend<std::io::Stderr>>;

pub struct Tui {
  pub terminal: ratatui::Terminal<Backend<std::io::Stderr>>,
}

impl Tui {
  pub fn new() -> Result<Self> {
    let terminal = Terminal::new(Backend::new(std::io::stderr()))?;
    Ok(Self { terminal })
  }

  pub fn enter(&self) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), EnterAlternateScreen, EnableMouseCapture, cursor::Hide)?;
    Ok(())
  }

  pub fn exit(&self) -> Result<()> {
    crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, DisableMouseCapture, cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
  }

  pub fn suspend(&self) -> Result<()> {
    self.exit()?;
    #[cfg(not(windows))]
    signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
    Ok(())
  }

  pub fn resume(&self) -> Result<()> {
    self.enter()?;
    Ok(())
  }
}
```

```admonish note
This is the same `Tui` struct we used in `initialize_panic_handler()`. We call `Tui::exit()` before printing the stacktrace.
```

Feel free to modify this as you need for use with `termion` or `wezterm`.

The type alias to `Frame` is only to make the `components` folder easier to work with, and is not strictly required.
