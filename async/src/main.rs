// ANCHOR: all
pub mod runner;

pub mod action;

pub mod components;

pub mod config;

pub mod tui;

pub mod utils;

use crate::{
  runner::Runner,
  utils::{initialize_logging, initialize_panic_handler, version},
};
use clap::Parser;
use color_eyre::eyre::Result;

//// ANCHOR: args
// Define the command line arguments structure
#[derive(Parser, Debug)]
#[command(version = version(), about = "ratatui async template with crossterm and tokio")]
struct Args {
  /// Tick rate (ms)
  #[arg(short, long, default_value_t = 250)]
  tick_rate: usize,
  /// Render tick rate (ms)
  #[arg(short, long, default_value_t = 100)]
  render_tick_rate: usize,
}
//// ANCHOR_END: args

#[tokio::main]
async fn main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler()?;

  let args = Args::parse();
  let mut runner = Runner::new((args.tick_rate, args.render_tick_rate))?;
  runner.run().await?;

  Ok(())
}
// ANCHOR_END: all
