use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
  #[arg(value_name = "FLOAT", help = "Tick rate, i.e. number of ticks per second", default_value_t = 1.0)]
  pub tick_rate: f64,

  #[arg(value_name = "FLOAT", help = "Frame rate, i.e. number of frames per second", default_value_t = 60.0)]
  pub frame_rate: f64,
}
