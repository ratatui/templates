// ANCHOR: all
pub mod runner;

pub mod action;

pub mod components;

pub mod config;

pub mod tui;

pub mod utils;

use clap::Parser;
use color_eyre::eyre::Result;

use crate::{
  runner::Runner,
  utils::{initialize_logging, initialize_panic_handler, version},
};

//// ANCHOR: args
// Define the command line arguments structure
#[derive(Parser, Debug)]
#[command(version = version(), about = "ratatui async template with crossterm and tokio")]
struct Args {
  /// Tick rate (n per second)
  #[arg(short, long, default_value_t = 4.0)]
  tick_rate: f64,
  /// Frame rate (n per second)
  #[arg(short, long, default_value_t = 60.0)]
  frame_rate: f64,
}
//// ANCHOR_END: args

async fn tokio_main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler()?;

  let args = Args::parse();
  let mut runner = Runner::new(args.tick_rate, args.frame_rate)?;
  runner.run().await?;

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  tokio_main().await.unwrap(); // Always invoke panic handler
  Ok(())
}
// ANCHOR_END: all
