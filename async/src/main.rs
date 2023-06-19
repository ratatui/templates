use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use ratatui_template::{app::App, logging::initialize_logging, tui::Tui, utils::initialize_panic_handler};
use tracing::error;

/// Ratatui Template TUI
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
  /// The tick rate to use
  #[arg(short, long, default_value_t = 50)]
  tick_rate: u64,
}

async fn tui_main(tick_rate: u64) -> Result<()> {
  let mut app = App::new(tick_rate)?;
  app.run().await?;
  Ok(())
}

fn main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler();

  let args = Args::parse();

  match tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()?
    .block_on(async { tui_main(args.tick_rate).await })
  {
    Ok(_) => std::process::exit(0),
    Err(e) => {
      match Tui::new() {
        Ok(tui) => {
          if let Err(r) = tui.exit() {
            error!("Unable to exit Tui: {r:?}");
          }
        },
        Err(r) => error!("Unable to exit Tui: {r:?}"),
      }
      let s = "Error".red().bold();
      eprintln!("{s}: {e}");
      std::process::exit(1)
    },
  }
}
