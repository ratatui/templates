use color_eyre::eyre::Result;
use tokio::sync::mpsc;

use crate::{
  action::Action,
  components::{app::App, fps::FpsCounter, Component},
  config::Config,
  tui::{self, Tui},
};

pub struct Runner {
  pub config: Config,
  pub tick_rate: (f64, f64),
  pub components: Vec<Box<dyn Component>>,
  pub should_quit: bool,
  pub should_suspend: bool,
}

impl Runner {
  pub fn new(tick_rate: (f64, f64)) -> Result<Self> {
    let h = App::new();
    let config = Config::new()?;
    let h = h.keymap(config.keymap.0.clone());
    let fps = FpsCounter::new();
    Ok(Self {
      tick_rate,
      components: vec![Box::new(h), Box::new(fps)],
      should_quit: false,
      should_suspend: false,
      config,
    })
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
        match e {
          tui::Event::Quit => action_tx.send(Action::Quit)?,
          tui::Event::Tick => action_tx.send(Action::Tick)?,
          tui::Event::Render => action_tx.send(Action::Render)?,
          tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
          e => {
            for component in self.components.iter_mut() {
              if let Some(action) = component.handle_events(Some(e.clone())) {
                action_tx.send(action)?;
              }
            }
          },
        }
      }

      while let Ok(action) = action_rx.try_recv() {
        if action != Action::Tick && action != Action::Render {
          log::debug!("{action:?}");
        }
        match action {
          Action::Quit => self.should_quit = true,
          Action::Suspend => self.should_suspend = true,
          Action::Resume => self.should_suspend = false,
          Action::Render => {
            tui.draw(|f| {
              for component in self.components.iter_mut() {
                component.draw(f, f.size());
              }
            })?;
          },
          _ => {},
        }
        for component in self.components.iter_mut() {
          if let Some(action) = component.update(action.clone())? {
            action_tx.send(action)?
          };
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
