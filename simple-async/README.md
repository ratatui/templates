# Ratatui Simple template

A simple example of a Ratatui application.

This is identical to the [simple](../simple) template but has `async` events out of the box with `tokio` and
`crossterm`'s `EventStream`.

## Design choices

We have a small `App` struct that has a main loop that calls methods to handle events and draw the
ui. The app can be quit by pressing any of Q/Esc/Ctrl+C.

We use [color-eyre](https://docs.rs/color-eyre/latest/color_eyre/) for simplifying any errors that
need to be reported to the console.
