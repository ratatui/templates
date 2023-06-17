use better_panic::Settings;
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
  }));
}
