use anyhow::Result;
use clap::Parser;
// use clap_complete::{generate, Shell as CompletionShell};
use ratatui_template::{
  app::App,
  utils::{initialize_logging, initialize_panic_handler, version},
};

#[derive(Parser, Debug)]
#[command(version = version(), about = "ratatui template with crossterm and tokio")]
struct Args {
  /// App tick rate
  #[arg(short, long, default_value_t = 50)]
  app_tick_rate: u64,
  /// Render tick rate
  #[arg(short, long, default_value_t = 50)]
  render_tick_rate: u64,
}

async fn tui_main(tick_rate: (u64, u64)) -> Result<()> {
  let mut app = App::new(tick_rate)?;
  app.run().await?;
  Ok(())
}

fn main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler();

  let args = Args::parse();

  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()?
    .block_on(async { tui_main((args.app_tick_rate, args.render_tick_rate)).await })
    .unwrap();

  Ok(())
}
