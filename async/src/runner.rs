use color_eyre::eyre::Result;
use tokio::sync::mpsc;

use crate::{
  action::Action,
  components::{app::App, Component},
  config::Config,
  trace_dbg,
  tui::Tui,
};

pub struct Runner {
  pub config: Config,
  pub tick_rate: (usize, usize),
  pub components: Vec<Box<dyn Component>>,
  pub should_quit: bool,
  pub should_suspend: bool,
}

impl Runner {
  pub fn new(tick_rate: (usize, usize)) -> Result<Self> {
    let h = App::new();
    let config = Config::new()?;
    let h = h.keymap(config.keymap.0.clone());
    Ok(Self { tick_rate, components: vec![Box::new(h)], should_quit: false, should_suspend: false, config })
  }

  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    let mut tui = Tui::new()?;
    tui.tick_rate(self.tick_rate);
    tui.enter()?;

    for component in self.components.iter_mut() {
      component.init(action_tx.clone())?;
    }

    loop {
      if let Some(e) = tui.next().await {
        for component in self.components.iter_mut() {
          if let Some(action) = component.handle_events(Some(e.clone())) {
            action_tx.send(action)?;
          }
        }
      }

      while let Ok(action) = action_rx.try_recv() {
        if action != Action::Tick {
          trace_dbg!(&action);
        }
        match action {
          Action::Quit => self.should_quit = true,
          Action::Suspend => self.should_suspend = true,
          Action::Resume => self.should_suspend = false,
          Action::Render => {
            for component in self.components.iter_mut() {
              tui.draw(|f| {
                component.draw(f, f.size());
              })?;
            }
            for component in self.components.iter_mut() {
              if let Some(action) = component.update(action.clone())? {
                action_tx.send(action)?
              };
            }
          },
          action => {
            for component in self.components.iter_mut() {
              if let Some(action) = component.update(action.clone())? {
                action_tx.send(action)?
              };
            }
          },
        }
      }
      if self.should_suspend {
        tui.suspend()?;
        action_tx.send(Action::Resume)?;
        tui = Tui::new()?;
        tui.tick_rate(self.tick_rate);
        tui.enter()?;
      } else if self.should_quit {
        tui.stop()?;
        break;
      }
    }
    tui.exit()?;
    Ok(())
  }
}
