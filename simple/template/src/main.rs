pub use app::App;

pub mod app;

/// Entry point of the application
fn main() -> color_eyre::Result<()> {
    // Install the panic handler
    color_eyre::install()?;

    // Run the application
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
