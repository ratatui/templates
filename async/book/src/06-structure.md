# `components/app.rs`

Here's an example of the `App` component with additional state:

1. `show_help` is a `bool` that tracks whether or not help should be rendered or not
1. `ticker` is a counter that increments every `AppTick`.

This `App` component also adds fields for `input: Input`, and stores a reference to
`action_tx: mpsc::UnboundedSender<Action>`

```rust,no_run,noplayground
{{#include ../../ratatui-counter/src/components/app.rs}}
```

The `render` function takes a `Frame` and draws a paragraph to display a counter as well as a text
box input:

![](https://user-images.githubusercontent.com/1813121/254134161-477b2182-a3ee-4be9-a180-1bcdc56c8a1d.png)

The `App` component has a couple of methods `increment` and `decrement` that we saw earlier, but
this time additional `Action`s are sent on the `action_tx` channel to track the start and end of the
increment.

```rust
  pub fn schedule_increment(&mut self, i: usize) {
    let tx = self.action_tx.clone().unwrap();
    tokio::task::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      tokio::time::sleep(Duration::from_secs(5)).await;
      tx.send(Action::Increment(i)).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }

  pub fn schedule_decrement(&mut self, i: usize) {
    let tx = self.action_tx.clone().unwrap();
    tokio::task::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      tokio::time::sleep(Duration::from_secs(5)).await;
      tx.send(Action::Decrement(i)).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }
```

When a `Action` is sent on the action channel, it is received in the `main` thread in the
`app.run()` loop which then calls the `dispatch` method with the appropriate action:

```rust
  fn dispatch(&mut self, action: Action) -> Option<Action> {
    match action {
      Action::Tick => self.tick(),
      Action::ToggleShowHelp => self.show_help = !self.show_help,
      Action::ScheduleIncrement=> self.schedule_increment(1),
      Action::ScheduleDecrement=> self.schedule_decrement(1),
      Action::Increment(i) => self.increment(i),
      Action::Decrement(i) => self.decrement(i),
      Action::EnterNormal => {
        self.mode = Mode::Normal;
      },
      Action::EnterInsert => {
        self.mode = Mode::Insert;
      },
      Action::EnterProcessing => {
        self.mode = Mode::Processing;
      },
      Action::ExitProcessing => {
        // TODO: Make this go to previous mode instead
        self.mode = Mode::Normal;
      },
      _ => (),
    }
    None
  }
```

This way, you can have `Action` affect multiple components by propagating the actions down all of
them.

When the `Mode` is switched to `Insert`, all events are handled off the `Input` widget from the
excellent [`tui-input` crate](https://github.com/sayanarijit/tui-input).

![](https://user-images.githubusercontent.com/1813121/254444604-de8cfcfa-eeec-417a-a8b0-92a7ccb5fcb5.gif)
