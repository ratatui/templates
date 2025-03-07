## Ratatui Event Driven Template (async)

This is a template for when you want to have an event-driven approach in your application.

The template will create the following project structure:

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── main.rs    -> entry-point
└── ui.rs      -> renders the widgets / UI
```

This is identical to the [event-driven](../event-driven) template but has `async` events out of the box with `tokio` and
`crossterm`'s `EventStream`.

## Design choices

We have a `App` struct that holds the logic. The `Widget` implementation for this struct lives in `ui` module and is responsible for rendering the UI.

The most crucial part of the application is the `event` module which contains the `Event`/`AppEvent` enums and `EventHandler` struct. The `EventHandler` struct spawns a new thread and listens for the events using [mpsc](https://doc.rust-lang.org/std/sync/mpsc/index.html) channels.

You can extend the `AppEvent` enum to include more custom events.

We use [color-eyre](https://docs.rs/color-eyre/latest/color_eyre/) for simplifying any errors that
need to be reported to the console.
