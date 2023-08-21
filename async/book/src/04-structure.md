# `action.rs`

Now that we have created a `Tui` and `EventHandler`, we are also going to introduce the `Command` pattern, i.e. the concept of "reified method calls" from <http://gameprogrammingpatterns.com/command.html>.
These are also typically called `Action`s or `Message`s.

```rust
pub enum Action {
  Quit,
  Tick,
  Increment,
  Decrement,
  Noop,
}
```

````admonish tip
You can attach payloads to enums in rust.
For example, in the following `Action` enum, `Increment(usize)` and `Decrement(usize)` have a `usize` payload which can be used to represent the value to add to or subtract from the counter as a payload.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
  Quit,
  Tick,
  Increment(usize),
  Decrement(usize),
  Noop,
}
```

````

Let's define a simple `impl App` such that every `Event` from the `EventHandler` is mapped to an `Action` from the enum.

```rust
#[derive(Default)]
struct App {
  counter: i64,
  should_quit: bool,
}

impl App {
  pub fn new() -> Self {
    Self::default()
  }

  pub async fn run(&mut self) -> Result<()> {
    let t = Tui::new();
    t.enter();
    let mut events = EventHandler::new(tick_rate);
    loop {
      let event = events.next().await;
      let action = self.handle_events(event);
      self.dispatch(action);
      t.terminal.draw(|f| self.render(f))?;
      if self.should_quit {
        break
      }
    };
    t.exit();
    Ok(())
  }

  fn handle_events(&mut self, event: Option<Event>) -> Action {
    match event {
      Some(Event::Quit) => Action::Quit,
      Some(Event::AppTick) => Action::Tick,
      Some(Event::Key(key_event)) => {
        if let Some(key) = event {
            match key.code {
              KeyCode::Char('q') => Action::Quit,
              KeyCode::Char('j') => Action::Increment,
              KeyCode::Char('k') => Action::Decrement
              _ => {}
          }
        }
      },
      Some(_) => Action::Noop,
      None => Action::Noop,
    }
  }

  fn dispatch(&mut self, action: Action) {
    match action {
      Action::Quit => self.should_quit = true,
      Action::Tick => self.tick(),
      Action::Increment => self.increment(),
      Action::Decrement => self.decrement(),
  }

  fn increment(&mut self) {
    self.counter += 1;
  }

  fn decrement(&mut self) {
    self.counter -= 1;
  }

  fn render(&mut self, f: &mut Frame<'_>) {
    f.render_widget(
      Paragraph::new(format!(
        "Press j or k to increment or decrement.\n\nCounter: {}",
        self.counter
      ))
    )
  }
}
```

We use `handle_events(event) -> Action` to take a `Event` and map it to a `Action`.
We use `dispatch(action)` to take an `Action` and modify the state of the app.

One advantage of this approach is that we can modify `handle_key_events()` to use a key configuration if we'd like, so that users can define their own map from key to action.

Another advantage of this is that the business logic of the `App` struct can be tested without having to create an instance of a `Tui` or `EventHandler`, e.g.:

```rust
mod tests {
  #[test]
  fn test_app() {
    let mut app = App::new();
    let old_counter = app.counter;
    app.dispatch(Action::Increment);
    assert!(app.counter == old_counter + 1);
  }
}
```

In the test above, we did not create an instance of the `Tui` or the `EventHandler`, and did not call the `run` function, but we are still able to test the business logic of our application.
Using "dispatching" on `Action`s gets us one step closer to making our application a "state machine", which improves understanding and testability.

If we wanted to be purist about it, we would make our `AppState` immutable, and we would have an `dispatch` function like so:

```rust
fn dispatch(app_state::AppState, action::Action) -> new_app_state::State {
  let mut state = app_state.clone();
  state.counter += 1;
  // ...
  state
}
```

In rare occasions, we may also want to choose a future action during `dispatch`.

```rust
fn dispatch(app_state::AppState, action::Action) -> (new_app_state::State, Option<action::Action>) {
  let mut state = app_state.clone();
  state.counter += 1;
  // ...
  (state, Action::Tick)
}
```

````admonish note
In [`Charm`'s `bubbletea`](https://github.com/charmbracelet/bubbletea), this function is called an `Update`. Here's an example of what that might look like:

```go
func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    switch msg := msg.(type) {

    // Is it a key press?
    case tea.KeyMsg:
        // These keys should exit the program.
        case "q":
            return m, tea.Quit

        case "k":
            m.counter--

        case "j":
            m.counter++
    }

    // Note that we're not returning a command.
    return m, nil
}
```

````

Writing code to follow this architecture in rust (in my opinion) requires more upfront design, mostly because you have to make your `AppState` struct `Clone`-friendly.
If I were in an exploratory or prototype stage of a TUI, I wouldn't want to do that and would only be interested in refactoring it this way once I got a handle on the design.

My workaround for this (as you saw earlier) is to make `dispatch` a method that takes a `&mut self`:

```rust
impl App {
  fn dispatch(&mut self, action: Action) -> Option<Action> {
    self.counter += 1
    None
  }
}
```

You are free to reorganize the code as you see fit!

You can also add more actions as required.
For example, here's all the actions in the template:

```rust,no_run,noplayground
{{#include ../../src/action.rs}}
```

```admonish note
We are choosing to use `serde` for `Action` so that we can allow users to decide which key event maps to which `Action` using a file for configuration.
This is discussed in more detail in the [`config`](./09-structure.md) section.
```
