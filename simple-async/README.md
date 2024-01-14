## Simple Async template

This simple async template will create the following project structure:

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
└── ui.rs      -> renders the widgets / UI
```

This is identical to the [simple] template but has `async` events out of the box with `tokio` and
`crossterm`'s `EventStream`.

[simple](../simple/)

Here's a `diff` if you use as reference if want to convert your own code to `async`:

**`./Cargo.toml`**

```diff
--- ./simple/Cargo.toml	2023-12-15 11:45:40
+++ ./simple-async/Cargo.toml	2024-01-14 05:33:25
@@ -6,5 +6,8 @@
 edition = "2021"

 [dependencies]
-crossterm = "0.27.0"
-ratatui = "0.24.0"
+crossterm = { version = "0.27.0", features = ["event-stream"] }
+futures = "0.3.30"
+ratatui = "0.25.0"
+tokio = { version = "1.35.1", features = ["full"] }
```

**`./src/event.rs`**

```diff
--- ./simple/src/event.rs	2024-01-06 22:25:37
+++ ./simple-async/src/event.rs	2024-01-14 05:42:04
@@ -1,8 +1,10 @@
+use std::time::Duration;
+
+use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
+use futures::{FutureExt, StreamExt};
+use tokio::sync::mpsc;
+
 use crate::app::AppResult;
-use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
-use std::sync::mpsc;
-use std::thread;
-use std::time::{Duration, Instant};

 /// Terminal events.
 #[derive(Clone, Copy, Debug)]
@@ -22,46 +24,53 @@
 #[derive(Debug)]
 pub struct EventHandler {
     /// Event sender channel.
-    sender: mpsc::Sender<Event>,
+    sender: mpsc::UnboundedSender<Event>,
     /// Event receiver channel.
-    receiver: mpsc::Receiver<Event>,
+    receiver: mpsc::UnboundedReceiver<Event>,
     /// Event handler thread.
-    handler: thread::JoinHandle<()>,
+    handler: tokio::task::JoinHandle<()>,
 }

 impl EventHandler {
     /// Constructs a new instance of [`EventHandler`].
     pub fn new(tick_rate: u64) -> Self {
         let tick_rate = Duration::from_millis(tick_rate);
-        let (sender, receiver) = mpsc::channel();
-        let handler = {
-            let sender = sender.clone();
-            thread::spawn(move || {
-                let mut last_tick = Instant::now();
+        let (sender, receiver) = mpsc::unbounded_channel();
+        let _sender = sender.clone();
+        let handler = tokio::spawn(async move {
+            let mut reader = crossterm::event::EventStream::new();
+            let mut tick = tokio::time::interval(tick_rate);
                 loop {
-                    let timeout = tick_rate
-                        .checked_sub(last_tick.elapsed())
-                        .unwrap_or(tick_rate);
-
-                    if event::poll(timeout).expect("failed to poll new events") {
-                        match event::read().expect("unable to read event") {
-                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
-                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
-                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
-                            CrosstermEvent::FocusGained => Ok(()),
-                            CrosstermEvent::FocusLost => Ok(()),
-                            CrosstermEvent::Paste(_) => unimplemented!(),
+                let tick_delay = tick.tick();
+                let crossterm_event = reader.next().fuse();
+                tokio::select! {
+                  _ = tick_delay => {
+                    _sender.send(Event::Tick).unwrap();
                         }
-                        .expect("failed to send terminal event")
+                  Some(Ok(evt)) = crossterm_event => {
+                    match evt {
+                      CrosstermEvent::Key(key) => {
+                        if key.kind == crossterm::event::KeyEventKind::Press {
+                          _sender.send(Event::Key(key)).unwrap();
                     }
-
-                    if last_tick.elapsed() >= tick_rate {
-                        sender.send(Event::Tick).expect("failed to send tick event");
-                        last_tick = Instant::now();
+                      },
+                      CrosstermEvent::Mouse(mouse) => {
+                        _sender.send(Event::Mouse(mouse)).unwrap();
+                      },
+                      CrosstermEvent::Resize(x, y) => {
+                        _sender.send(Event::Resize(x, y)).unwrap();
+                      },
+                      CrosstermEvent::FocusLost => {
+                      },
+                      CrosstermEvent::FocusGained => {
+                      },
+                      CrosstermEvent::Paste(_) => {
+                      },
                     }
                 }
-            })
                 };
+            }
+        });
         Self {
             sender,
             receiver,
@@ -73,7 +82,13 @@
     ///
     /// This function will always block the current thread if
     /// there is no data available and it's possible for more data to be sent.
-    pub fn next(&self) -> AppResult<Event> {
-        Ok(self.receiver.recv()?)
+    pub async fn next(&mut self) -> AppResult<Event> {
+        self.receiver
+            .recv()
+            .await
+            .ok_or(Box::new(std::io::Error::new(
+                std::io::ErrorKind::Other,
+                "This is an IO error",
+            )))
     }
 }
```

**`./src/main.rs`**

```diff
diff -bur ./simple/src/main.rs ./simple-async/src/main.rs
--- ./simple/src/main.rs	2023-12-15 11:45:41
+++ ./simple-async/src/main.rs	2024-01-14 05:36:37
@@ -6,7 +6,8 @@
 use ratatui::backend::CrosstermBackend;
 use ratatui::Terminal;

-fn main() -> AppResult<()> {
+#[tokio::main]
+async fn main() -> AppResult<()> {
     // Create an application.
     let mut app = App::new();

@@ -22,7 +23,7 @@
         // Render the user interface.
         tui.draw(&mut app)?;
         // Handle events.
-        match tui.events.next()? {
+        match tui.events.next().await? {
             Event::Tick => app.tick(),
             Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
             Event::Mouse(_) => {}

```
