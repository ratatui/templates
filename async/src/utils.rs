use std::path::PathBuf;

use better_panic::Settings;
use colored::Colorize;
use directories::ProjectDirs;
use shadow_rs::shadow;
use tracing::error;

shadow!(build);

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

pub fn version_msg() -> String {
  let version = clap::crate_version!();
  // let author = clap::crate_authors!();
  // let home_page = env!("CARGO_PKG_HOMEPAGE");

  let commit_date = build::COMMIT_DATE;
  let commit_hash = build::SHORT_COMMIT;
  let build_time = build::BUILD_TIME;
  let build_target = build::BUILD_TARGET;

  let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
  let config_dir_path = get_config_dir().display().to_string();
  let data_dir_path = get_data_dir().display().to_string();

  format!(
    "\
{version}

Commit date: {commit_date}
Commit hash: {commit_hash}
Build time: {build_time}
Build target: {build_target}

Executable path: {current_exe_path}
Config directory: {config_dir_path}
Data directory: {data_dir_path}"
  )
}
