# `app.rs`

Finally, putting all the pieces together, we are almost ready to get the `App` struct. Before we do,
we should discuss the process of a TUI.

Most TUIs are single process, single threaded applications.

```svgbob
 ,-------------.
 |Get Key Event|
 `-----+-------'
       |
       |
 ,-----v------.
 |Update State|
 `-----+------'
       |
       |
   ,---v---.
   | Draw  |
   `-------'
```

When an application is structured like this, the TUI is blocking at each step:

1. Waiting for a Event.
   - If no key or mouse event in 250ms, send `Tick`.
2. Update the state of the app based on `event` or `action`.
3. `draw` the state of the app to the terminal using `ratatui`.

This works perfectly fine for small applications, and this is what I recommend starting out with.
For _most_ TUIs, you'll never need to graduate from this process methodology.

Usually, `draw` and `get_events` are fast enough that it doesn't matter. But if you do need to do a
computationally demanding or I/O intensive task while updating state (e.g. reading a database,
computing math or making a web request), your app may "hang" while it is doing so.

Let's say a user presses `j` to scroll down a list. And every time the user presses `j` you want to
check the web for additional items to add to the list.

What should happen when a user presses and holds `j`? It is up to you to decide how you would like
your TUI application to behave in that instance.

You may decide that the desired behavior for your app is to hang while downloading new elements for
the list, and all key presses while the app hangs are received and handled "instantly" after the
download completes.

Or you may decide to `flush` all keyboard events so they are not buffered, and you may want to
implement something like the following:

```rust
let mut app = App::new();
loop {
  // ...
  let before_draw = Instant::now();
  t.terminal.draw(|f| self.render(f))?;
  // If drawing to the terminal is slow, flush all keyboard events so they're not buffered.
  if before_draw.elapsed() > Duration::from_millis(20) {
      while let Ok(_) = events.try_next() {}
  }
  // ...
}
```

Alternatively, you may decide you want the app to update in the background, and a user should be
able to scroll through the existing list while the app is downloading new elements.

In my experience, the trade-off is here is usually complexity for the developer versus ergonomics
for the user.

Let's say we weren't worried about complexity, and were interested in performing a computationally
demanding or I/O intensive task in the background. For our example, let's say that we wanted to
trigger a increment to the counter after sleeping for `5` seconds.

This means that we'll have to start a "task" that sleeps for 5 seconds, and then sends another
`Action` to be dispatched on.

Now, our `dispatch()` method takes the following shape:

```rust
  fn dispatch(&mut self, action: Action) -> Option<Action> {
    match action {
      Action::Tick => self.tick(),
      Action::ScheduleIncrement => self.schedule_increment(1),
      Action::ScheduleDecrement => self.schedule_decrement(1),
      Action::Increment(i) => self.increment(i),
      Action::Decrement(i) => self.decrement(i),
      _ => (),
    }
    None
  }
```

And `schedule_increment()` and `schedule_decrement()` both spawn short lived `tokio` tasks:

```rust
  pub fn schedule_increment(&mut self, i: i64) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tokio::time::sleep(Duration::from_secs(5)).await;
      tx.send(Action::Increment(i)).unwrap();
    });
  }

  pub fn schedule_decrement(&mut self, i: i64) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tokio::time::sleep(Duration::from_secs(5)).await;
      tx.send(Action::Decrement(i)).unwrap();
    });
  }

  pub fn increment(&mut self, i: i64) {
    self.counter += i;
  }

  pub fn decrement(&mut self, i: i64) {
    self.counter -= i;
  }

```

In order to do this, we want to set up a `action_tx` on the `App` struct:

```rust
#[derive(Default)]
struct App {
  counter: i64,
  should_quit: bool,
  action_tx: Option<UnboundedSender<Action>>
}
```

```admonish note
The only reason we are using an `Option<>` here for `action_tx` is that we are not initializing the action channel when creating the instance of the `App`.
```

This is what we want to do:

```rust
  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();
    let t = Tui::new();
    t.enter();

    tokio::spawn(async move {
      let mut event = EventHandler::new(250);
      loop {
        let event = event.next().await;
        let action = self.handle_events(event); // ERROR: self is moved to this tokio task
        action_tx.send(action);
      }
    })

    loop {
      if let Some(action) = action_rx.recv().await {
        self.dispatch(action);
      }
      t.terminal.draw(|f| self.render(f))?;
      if self.should_quit {
        break
      }
    }
    t.exit();
    Ok(())
  }
```

However, this doesn't quite work because we can't move `self`, i.e. the `App` to the
`event -> action` mapping, i.e. `self.handle_events()`, and still use it later for
`self.dispatch()`. One way to solve this is to pass a `Arc<Mutex<Component>` instance to the
`event -> action` mapping loop, where it uses a `lock()` to get a reference to the object to call
`obj.handle_events()`. We'll have to use the same `lock()` functionality in the main loop as well to
call `obj.dispatch()`.

```rust
pub struct App {
  pub component: Arc<Mutex<Component>>,
  pub should_quit: bool,
}

impl App {
  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    let tui = Tui::new();
    tui.enter();

    tokio::spawn(async move {
      let component = self.component.clone();
      let mut event = EventHandler::new(250);
      loop {
        let event = event.next().await;
        let action = component.lock().await.handle_events(event);
        action_tx.send(action);
      }
    })

    loop {
      if let Some(action) = action_rx.recv().await {
        match action {
          Action::RenderTick => {
            let c = self.component.lock().await;
            t.terminal.draw(|f| c.render(f))?;
          };
          Action::Quit => self.should_quit = true,
          _ => self.component.lock().await.dispatch(action),
        }
      }
      self.should_quit {
        break;
      }
    }

    tui.exit();
    Ok(())
  }
}
```

Now our `App` is generic boilerplate that doesn't depend on any business logic. It is responsible
just to drive the application forward, i.e. call appropriate functions.

All business logic will be located in a `Component` struct.

```rust
#[derive(Default)]
struct Component {
  counter: i64,
}

impl Component {
  fn handle_events(&mut self, event: Option<Event>) -> Action {
    match event {
      Some(Event::Quit) => Action::Quit,
      Some(Event::AppTick) => Action::Tick,
      Some(Event::RenderTick) => Action::RenderTick,
      Some(Event::Key(key_event)) => {
        if let Some(key) = event {
            match key.code {
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

We can go one step further and make the render loop its own `tokio` task:

```rust
pub struct App {
  pub component: Arc<Mutex<Component>>,
  pub should_quit: bool,
}

impl App {
  pub async fn run(&mut self) -> Result<()> {
    let (render_tx, mut render_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
      let component = self.component.clone();
      let tui = Tui::new();
      tui.enter();
      loop {
        if let Some(_) = render_rx.recv() {
          let c = self.component.lock().await;
          tui.terminal.draw(|f| c.render(f))?;
        }
      }
      tui.exit()
    })

    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
      let component = self.component.clone();
      let mut event = EventHandler::new(250);
      loop {
        let event = event.next().await;
        let action = component.lock().await.handle_events(event);
        action_tx.send(action);
      }
    })

    loop {
      if let Some(action) = action_rx.recv().await {
        match action {
          Action::RenderTick => {
            render_tx.send(());
          };
          Action::Quit => self.should_quit = true,
          _ => self.component.lock().await.dispatch(action),
        }
      }
      self.should_quit {
        break;
      }
    }

    Ok(())
  }
}
```

Our final architecture looks like this:

```svgbob
    Render Thread               Event Thread                  Main Thread

                             ,------------------.
                             |  Get Key Event   |
                             `--------+---------'
                                      |
                            ,---------v-----------.
                            | Map Event to Action |
                            `---------+-----------'
                                      |
                         ,------------V--------------.        ,-------------.
                         | Send Action on action_tx  |------->| Recv Action |
                         `---------------------------´        `-----+-------´
                                                                    |
,-------------------.                                      ,--------v--------.
| Recv on render_rx |<-------------------------------------| Dispatch Action |
`--------+----------´                                      `--------+--------´
         |                                                          |
,--------v---------.                                       ,--------v---------.
| Render Component |                                       | Update Component |
`------------------´                                       `------------------´
```

You can change around when "thread" or "task" does what in your application if you'd like.

I personally like to create a `TerminalHandler` that handles the render thread task.

```rust,no_run,noplayground
{{#include ../../src/terminal.rs:terminal_handler}}
```

And I like to update the `EventHandler` itself to map the event to an action and send that over the
action channel, like so:

```rust,no_run,noplayground
{{#include ../../src/event.rs:event}}
```

With that, our `App` becomes a little more simpler:

```rust
pub struct App {
  pub tick_rate: (u64, u64),
  pub component: Arc<Mutex<Component>>,
  pub should_quit: bool,
}

impl App {
  pub fn new(tick_rate: (u64, u64)) -> Result<Self> {
    let component = Arc::new(Mutex::new(Component::new()));
    Ok(Self { tick_rate, component, should_quit: false, should_suspend: false })
  }

  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    let mut terminal = TerminalHandler::new(self.component.clone());
    let mut event = EventHandler::new(self.tick_rate, self.component.clone(), action_tx.clone());

    loop {
      if let Some(action) = action_rx.recv().await {
        match action {
          Action::RenderTick => terminal.render()?,
          Action::Quit => self.should_quit = true,
          _ => self.component.lock().await.dispatch(action),
        }
      }
      if self.should_quit {
        terminal.stop()?;
        event.stop();
        terminal.task.await?;
        event.task.await?;
        break;
      }
    }
    Ok(())
  }
}
```

Our `Component` currently does one thing and just one thing (increment and decrement a counter). But
we may want to do more complex things and combine `Component`s in interesting ways. For example, we
may want to add a text input field as well as show logs conditionally from our TUI application.

In the next sections, we will talk about breaking out our app into various components, with the one
root component called `Home`. And we'll introduce a `Component` trait so it is easier to understand
where the TUI specific code ends and where our app's business logic begins.
