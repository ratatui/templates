use anyhow::Result;
use clap::Parser;
// use clap_complete::{generate, Shell as CompletionShell};
use colored::Colorize;
use ratatui_template::{app::App, logging::initialize_logging, tui::Tui, utils::initialize_panic_handler};
use shadow_rs::{formatcp, shadow};
use tracing::error;

shadow!(build);

const VERSION_INFO: &str = formatcp!(
  r#"{}
commit_hash: {}
build_time: {}
build_env: {}, {}"#,
  build::PKG_VERSION,
  build::SHORT_COMMIT,
  build::BUILD_TIME,
  build::RUST_VERSION,
  build::RUST_CHANNEL
);

#[derive(Parser, Debug)]
#[command(author=clap::crate_authors!(), version=VERSION_INFO, about = "ratatui template with crossterm and tokio")]
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
