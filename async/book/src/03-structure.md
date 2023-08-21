# `event.rs`

In it's simplest form, most applications will have a `main` loop like this:

```rust
fn main() -> Result<()> {
  let mut app = App::new();

  let mut t = Tui::new()?;

  t.enter()?; // raw mode enabled

  loop {

    // get key event and update state
    // ... Special handling to read key or mouse events required here

    t.terminal.draw(|f| { // <- `terminal.draw` is the only ratatui function here
      ui(app, f) // render state to terminal
    })?;

  }

  t.exit()?; // raw mode disabled

  Ok(())
}
```

```admonish note

The `terminal.draw(|f| { ui(app, f); })` call is the only line in the code above that uses `ratatui` functionality.
You can learn more about [`draw` from the official documentation](https://docs.rs/ratatui/latest/ratatui/terminal/struct.Terminal.html#method.draw).
Essentially, `terminal.draw()` takes a callback that takes a [`Frame`](https://docs.rs/ratatui/latest/ratatui/terminal/struct.Frame.html) and expects the callback to render widgets to that frame, which is then drawn to the terminal using a double buffer technique.

```

While we are in the "raw mode", i.e. after we call `t.enter()`, any key presses in that terminal window are sent to `stdin`.
We have to read these key presses from `stdin` if we want to act on them.

There's a number of different ways to do that.
`crossterm` has a `event` module that implements features to read these key presses for us.

Let's assume we were building a simple "counter" application, that incremented a counter when we pressed `j` and decremented a counter when we pressed `k`.

```rust
fn main() -> Result {
  let mut app = App::new();

  let mut t = Tui::new()?;

  t.enter()?;

  loop {
    if crossterm::event::poll(Duration::from_millis(250))? {
      if let Event::Key(key) = crossterm::event::read()? {
        match key.code {
          KeyCode::Char('j') => app.increment(),
          KeyCode::Char('k') => app.decrement(),
          KeyCode::Char('q') => break,
          _ => (),
        }
      }
    };

    t.terminal.draw(|f| {
      ui(app, f)
    })?;
  }

  t.exit()?;

  Ok(())
}
```

This works perfectly fine, and a lot of small to medium size programs can get away with doing just that.

However, this approach conflates the key input handling with app state updates, and does so in the "draw" loop.
The practical issue with this approach is we block the draw loop for 250 ms waiting for a key press.
This can have odd side effects, for example pressing an holding a key will result in faster draws to the terminal.

In terms of architecture, the code could get complicated to reason about.
For example, we may even want key presses to mean _different_ things depending on the state of the app (when you are focused on an input field, you may want to enter the letter `"j"` into the text input field, but when focused on a list of items, you may want to scroll down the list.)

![Pressing `j` 3 times to increment counter and 3 times in the text field](https://user-images.githubusercontent.com/1813121/254444604-de8cfcfa-eeec-417a-a8b0-92a7ccb5fcb5.gif)

<!--
```
Set Shell zsh
Sleep 1s
Hide
Type "cargo run"
Enter
Sleep 1s
Show
Type "jjj"
Sleep 5s
Sleep 5s
Type "/jjj"
Sleep 5s
Escape
Type "q"
```
-->

We have to do a few different things set ourselves up, so let's take things one step at a time.

First, instead of polling, we are going to introduce channels to get the key presses asynchronously and send them over a channel.
We will then receive on the channel in the `main` loop.

There are two ways to do this.
We can either use OS threads or "green" threads, i.e. tasks, i.e. rust's `async`-`await` features + a future executor.

Here's example code of reading key presses asynchronously using `std::thread` and `tokio::task`.

## `std::thread`

```rust
enum Event {
  Key(crossterm::event::KeyEvent)
}

struct EventHandler {
  rx: std::sync::mpsc::Receiver<Event>,
}

impl EventHandler {
  fn new() -> Self {
    let tick_rate = std::time::Duration::from_millis(250);
    let (tx, rx) =  std::sync::mpsc::channel();
    std::thread::spawn(move || {
      loop {
        if crossterm::event::poll(tick_rate)? {
          match crossterm::event::read()? {
            CrosstermEvent::Key(e) => tx.send(Event::Key(e)),
            _ => unimplemented!(),
          }?
        }
      }
    })

    EventHandler { rx }
  }

  fn next(&self) -> Result<Event> {
    Ok(self.rx.recv()?)
  }
}
```

## `tokio::task`

```rust
enum Event {
  Key(crossterm::event::KeyEvent)
}

struct EventHandler {
  rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
  fn new() -> Self {
    let tick_rate = std::time::Duration::from_millis(250);
    let (tx, mut rx) =  tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
      loop {
        if crossterm::event::poll(tick_rate)? {
          match crossterm::event::read()? {
            CrosstermEvent::Key(e) => tx.send(Event::Key(e)),
            _ => unimplemented!(),
          }?
        }
      }
    })

    EventHandler { rx }
  }

  async fn next(&self) -> Result<Event> {
    Ok(self.rx.recv().await.ok()?)
  }
}
```

## `diff`

```diff
  enum Event {
    Key(crossterm::event::KeyEvent)
  }

  struct EventHandler {
-   rx: std::sync::mpsc::Receiver<Event>,
+   rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
  }

  impl EventHandler {
    fn new() -> Self {
      let tick_rate = std::time::Duration::from_millis(250);
-     let (tx, rx) =  std::sync::mpsc::channel();
+     let (tx, mut rx) =  tokio::sync::mpsc::unbounded_channel();
-     std::thread::spawn(move || {
+     tokio::spawn(async move {
        loop {
          if crossterm::event::poll(tick_rate)? {
            match crossterm::event::read()? {
              CrosstermEvent::Key(e) => tx.send(Event::Key(e)),
              _ => unimplemented!(),
            }?
          }
        }
      })

      EventHandler { rx }
    }

-   fn next(&self) -> Result<Event> {
+   async fn next(&self) -> Result<Event> {
-     Ok(self.rx.recv()?)
+     Ok(self.rx.recv().await.ok()?)
    }
  }
```

````admonish warning

A lot of examples out there in the wild might use the following code for sending key presses:


```rust
  CrosstermEvent::Key(e) => tx.send(Event::Key(e)),
```

However, on Windows, when using `Crossterm`, this will send the same `Event::Key(e)` twice; one for when you press the key, i.e. `KeyEventKind::Press` and one for when you release the key, i.e. `KeyEventKind::Release`.
On `MacOS` and `Linux` only `KeyEventKind::Press` kinds of `key` event is generated.

To make the code work as expected across all platforms, you can do this instead:

```rust
  CrosstermEvent::Key(key) => {
    if key.kind == KeyEventKind::Press {
      event_tx.send(Event::Key(key)).unwrap();
    }
  },
```

````

Tokio is an asynchronous runtime for the Rust programming language.
It is one of the more popular runtimes for asynchronous programming in rust.
You can learn more about here <https://tokio.rs/tokio/tutorial>.
For the rest of the tutorial here, we are going to assume we want to use tokio.
I highly recommend you read the official `tokio` documentation.

If we use `tokio`, receiving a event requires `.await`. So our `main` loop now looks like this:

```rust
#[tokio::main]
async fn main() -> {
  let mut app = App::new();

  let events = EventHandler::new();

  let mut t = Tui::new()?;

  t.enter()?;

  loop {
    if let Event::Key(key) = events.next().await? {
      match key.code {
        KeyCode::Char('j') => app.increment(),
        KeyCode::Char('k') => app.decrement(),
        KeyCode::Char('q') => break,
        _ => (),
      }
    }

    t.terminal.draw(|f| {
      ui(app, f)
    })?;
  }

  t.exit()?;

  Ok(())
}
```

### Additional improvements

We are going to modify our `EventHandler` to handle a `AppTick` event.
We want the `Event::AppTick` to be sent at regular intervals.
We are also going to want to use a `CancellationToken` to stop the tokio task on request.

[`tokio`'s `select!` macro](https://tokio.rs/tokio/tutorial/select) allows us to wait on multiple `async` computations and returns when a single computation completes.

Here's what the completed `EventHandler` code now looks like:

```rust,no_run,noplayground
use anyhow::Result;
use crossterm::{
  cursor,
  event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent},
};
use futures::{FutureExt, StreamExt};
use tokio::{
  sync::{mpsc, oneshot},
  task::JoinHandle,
};

#[derive(Clone, Copy, Debug)]
pub enum Event {
  Error,
  AppTick,
  Key(KeyEvent),
}

#[derive(Debug)]
pub struct EventHandler {
  _tx: mpsc::UnboundedSender<Event>,
  rx: mpsc::UnboundedReceiver<Event>,
  task: Option<JoinHandle<()>>,
  stop_cancellation_token: CancellationToken,
}

impl EventHandler {
  pub fn new(tick_rate: u64) -> Self {
    let tick_rate = std::time::Duration::from_millis(tick_rate);

    let (tx, rx) = mpsc::unbounded_channel();
    let _tx = tx.clone();

    let stop_cancellation_token = CancellationToken::new();
    let _stop_cancellation_token = stop_cancellation_token.clone();

    let task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);
      loop {
        let delay = interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          _ = _stop_cancellation_token.cancelled() => {
            break;
          }
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                      tx.send(Event::Key(key)).unwrap();
                    }
                  },
                  _ => {},
                }
              }
              Some(Err(_)) => {
                tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = delay => {
              tx.send(Event::AppTick).unwrap();
          },
        }
      }
    });

    Self { _tx, rx, task: Some(task), stop_cancellation_token }
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.rx.recv().await
  }

  pub async fn stop(&mut self) -> Result<()> {
    self.stop_cancellation_token.cancel();
    if let Some(handle) = self.task.take() {
      handle.await.unwrap();
    }
    Ok(())
  }
}
```

````admonish note

Using `crossterm::event::EventStream::new()` requires the `event-stream` feature to be enabled.

```yml
crossterm = { version = "0.26.1", default-features = false, features = [
"event-stream",
] }
```

````

With this `EventHandler` implemented, we can use `tokio` to create a separate "task" that handles any key asynchronously in our `main` loop.

In the next section, we will introduce a `Command` pattern to bridge handling the effect of an event.
