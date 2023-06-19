use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
// use clap_complete::{generate, Shell as CompletionShell};
use colored::Colorize;
use ratatui_template::{
  app::App,
  logging::initialize_logging,
  tui::Tui,
  utils::{get_config_dir, initialize_panic_handler},
};
use tracing::error;

pub fn version() -> String {
  let version = clap::crate_version!();
  let author = clap::crate_authors!();

  let commit_hash = env!("RATATUI_TEMPLATE_GIT_INFO");

  let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
  let config_dir_path = get_config_dir().display().to_string();
  let data_dir_path = ratatui_template::utils::get_data_dir().display().to_string();

  format!(
    "\
{version} - ({commit_hash})

Authors: {author}
Executable path: {current_exe_path}
Config directory: {config_dir_path}
Data directory: {data_dir_path}"
  )
}

#[derive(Parser, Debug)]
#[command(version = version(), about = "ratatui template with crossterm and tokio")]
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
    Ok(_) => std::process::exit(libc::EXIT_SUCCESS),
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
      std::process::exit(libc::EXIT_FAILURE)
    },
  }
}
