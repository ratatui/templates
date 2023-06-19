use std::path::PathBuf;

use better_panic::Settings;
use colored::Colorize;
use directories::ProjectDirs;
use tracing::error;

use crate::tui::Tui;

pub fn initialize_panic_handler() {
  std::panic::set_hook(Box::new(|panic_info| {
    match Tui::new() {
      Ok(tui) => {
        if let Err(r) = tui.exit() {
          error!("Unable to exit Tui: {r:?}");
        }
      },
      Err(r) => error!("Unable to exit Tui: {r:?}"),
    }
    Settings::auto().most_recent_first(false).lineno_suffix(true).create_panic_handler()(panic_info);
    std::process::exit(libc::EXIT_FAILURE);
  }));
}

pub fn get_data_dir() -> PathBuf {
  let directory = if let Ok(s) = std::env::var("RATATUI_TEMPLATE_DATA") {
    PathBuf::from(s)
  } else if let Some(proj_dirs) = ProjectDirs::from("com", "kdheepak", "ratatui-template") {
    proj_dirs.data_local_dir().to_path_buf()
  } else {
    let s = "Error".red().bold();
    eprintln!("{s}: Unable to find data directory for ratatui-template");
    std::process::exit(libc::EXIT_FAILURE)
  };
  directory
}

pub fn get_config_dir() -> PathBuf {
  let directory = if let Ok(s) = std::env::var("RATATUI_TEMPLATE_CONFIG") {
    PathBuf::from(s)
  } else if let Some(proj_dirs) = ProjectDirs::from("com", "kdheepak", "ratatui-template") {
    proj_dirs.config_local_dir().to_path_buf()
  } else {
    let s = "Error".red().bold();
    eprintln!("{s}: Unable to find data directory for ratatui-template");
    std::process::exit(libc::EXIT_FAILURE)
  };
  directory
}
