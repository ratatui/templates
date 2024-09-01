# Ratatui Simple template

The simple template will create the following project structure:

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── main.rs    -> entry-point
```

## Design choices

We use [color-eyre](https://docs.rs/color-eyre/latest/color_eyre/) for simplifying any errors that
need to be reported to the console.

We have a small `App` struct that has a main loop that calls methods to handle events and draw the
ui.

Events are read on a secondary thread and collated into a single mpsc channel. This allows a tick
event to be interspersed with other events like keyboard and mouse. The Tick event is a useful place
to perform updates to animations or to do external polling.

The app can be quit by pressing any of Q/Esc/Ctrl+C.
