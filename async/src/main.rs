// ANCHOR: all
use anyhow::Result;
use clap::Parser;
use ratatui_async_template::{
  app::App,
  utils::{initialize_logging, initialize_panic_handler, version},
};

//// ANCHOR: args
// Define the command line arguments structure
#[derive(Parser, Debug)]
#[command(version = version(), about = "ratatui template with crossterm and tokio")]
struct Args {
  /// App tick rate
  #[arg(short, long, default_value_t = 1000)]
  app_tick_rate: u64,
  /// Render tick rate
  #[arg(short, long, default_value_t = 50)]
  render_tick_rate: u64,
}
//// ANCHOR_END: args

#[tokio::main]
async fn main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler();

  let args = Args::parse();
  let tick_rate = (args.app_tick_rate, args.render_tick_rate);

  let mut app = App::new(tick_rate)?;
  app.run().await?;

  Ok(())
}
// ANCHOR_END: all
